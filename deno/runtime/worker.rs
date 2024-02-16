// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_cache::CreateCache;
use deno_cache::SqliteBackedCache;
use deno_core::ascii_str;
use deno_core::error::AnyError;
use deno_core::error::JsError;
use deno_core::merge_op_metrics;
use deno_core::v8;
use deno_core::CompiledWasmModuleStore;
use deno_core::Extension;
use deno_core::FeatureChecker;
use deno_core::FsModuleLoader;
use deno_core::GetErrorClassFn;
use deno_core::JsRuntime;
use deno_core::LocalInspectorSession;
use deno_core::ModuleCodeString;
use deno_core::ModuleId;
use deno_core::ModuleLoader;
use deno_core::ModuleSpecifier;
use deno_core::OpMetricsFactoryFn;
use deno_core::OpMetricsSummaryTracker;
use deno_core::PollEventLoopOptions;
use deno_core::RuntimeOptions;
use deno_core::SharedArrayBufferStore;
use deno_core::Snapshot;
use deno_core::SourceMapGetter;
use deno_cron::local::LocalCronHandler;
use deno_fs::FileSystem;
use deno_http::DefaultHttpPropertyExtractor;
use deno_io::Stdio;
use deno_kv::dynamic::MultiBackendDbHandler;
use deno_tls::RootCertStoreProvider;
use deno_web::BlobStore;
use log::debug;

use crate::inspector_server::InspectorServer;
use crate::ops;
use crate::permissions::PermissionsContainer;
use crate::shared::runtime;
use crate::BootstrapOptions;

pub type FormatJsErrorFn = dyn Fn(&JsError) -> String + Sync + Send;

pub fn import_meta_resolve_callback(
  loader: &dyn deno_core::ModuleLoader,
  specifier: String,
  referrer: String,
) -> Result<ModuleSpecifier, AnyError> {
  loader.resolve(
    &specifier,
    &referrer,
    deno_core::ResolutionKind::DynamicImport,
  )
}

// TODO(bartlomieju): temporary measurement until we start supporting more
// module types
pub fn validate_import_attributes_callback(
  scope: &mut v8::HandleScope,
  attributes: &HashMap<String, String>,
) {
  for (key, value) in attributes {
    let msg = if key != "type" {
      Some(format!("\"{key}\" attribute is not supported."))
    } else if value != "json" {
      Some(format!("\"{value}\" is not a valid module type."))
    } else {
      None
    };

    let Some(msg) = msg else {
      continue;
    };

    let message = v8::String::new(scope, &msg).unwrap();
    let exception = v8::Exception::type_error(scope, message);
    scope.throw_exception(exception);
    return;
  }
}

#[derive(Clone, Default)]
pub struct ExitCode(Arc<AtomicI32>);

impl ExitCode {
  pub fn get(&self) -> i32 {
    self.0.load(Relaxed)
  }

  pub fn set(&mut self, code: i32) {
    self.0.store(code, Relaxed);
  }
}

/// This worker is created and used by almost all
/// subcommands in Deno executable.
///
/// It provides ops available in the `Deno` namespace.
///
/// All `WebWorker`s created during program execution
/// are descendants of this worker.
pub struct MainWorker {
  pub js_runtime: JsRuntime,
  should_break_on_first_statement: bool,
  should_wait_for_inspector_session: bool,
  exit_code: ExitCode,
  bootstrap_fn_global: Option<v8::Global<v8::Function>>,
}

pub struct WorkerOptions {
  pub bootstrap: BootstrapOptions,

  /// JsRuntime extensions, not to be confused with ES modules.
  ///
  /// Extensions register "ops" and JavaScript sources provided in `js` or `esm`
  /// configuration. If you are using a snapshot, then extensions shouldn't
  /// provide JavaScript sources that were already snapshotted.
  pub extensions: Vec<Extension>,

  /// V8 snapshot that should be loaded on startup.
  pub startup_snapshot: Option<Snapshot>,

  /// Should op registration be skipped?
  pub skip_op_registration: bool,

  /// Optional isolate creation parameters, such as heap limits.
  pub create_params: Option<v8::CreateParams>,

  pub unsafely_ignore_certificate_errors: Option<Vec<String>>,
  pub root_cert_store_provider: Option<Arc<dyn RootCertStoreProvider>>,
  pub seed: Option<u64>,

  pub fs: Arc<dyn FileSystem>,
  /// Implementation of `ModuleLoader` which will be
  /// called when V8 requests to load ES modules.
  ///
  /// If not provided runtime will error if code being
  /// executed tries to load modules.
  pub module_loader: Rc<dyn ModuleLoader>,
  pub npm_resolver: Option<Arc<dyn deno_node::NpmResolver>>,
  // Callbacks invoked when creating new instance of WebWorker
  pub create_web_worker_cb: Arc<ops::worker_host::CreateWebWorkerCb>,
  pub format_js_error_fn: Option<Arc<FormatJsErrorFn>>,

  /// Source map reference for errors.
  pub source_map_getter: Option<Box<dyn SourceMapGetter>>,
  pub maybe_inspector_server: Option<Arc<InspectorServer>>,
  // If true, the worker will wait for inspector session and break on first
  // statement of user code. Takes higher precedence than
  // `should_wait_for_inspector_session`.
  pub should_break_on_first_statement: bool,
  // If true, the worker will wait for inspector session before executing
  // user code.
  pub should_wait_for_inspector_session: bool,
  /// If Some, print a low-level trace output for ops matching the given patterns.
  pub strace_ops: Option<Vec<String>>,

  /// Allows to map error type to a string "class" used to represent
  /// error in JavaScript.
  pub get_error_class_fn: Option<GetErrorClassFn>,
  pub cache_storage_dir: Option<std::path::PathBuf>,
  pub origin_storage_dir: Option<std::path::PathBuf>,
  pub blob_store: Arc<BlobStore>,
  pub broadcast_channel: InMemoryBroadcastChannel,

  /// The store to use for transferring SharedArrayBuffers between isolates.
  /// If multiple isolates should have the possibility of sharing
  /// SharedArrayBuffers, they should use the same [SharedArrayBufferStore]. If
  /// no [SharedArrayBufferStore] is specified, SharedArrayBuffer can not be
  /// serialized.
  pub shared_array_buffer_store: Option<SharedArrayBufferStore>,

  /// The store to use for transferring `WebAssembly.Module` objects between
  /// isolates.
  /// If multiple isolates should have the possibility of sharing
  /// `WebAssembly.Module` objects, they should use the same
  /// [CompiledWasmModuleStore]. If no [CompiledWasmModuleStore] is specified,
  /// `WebAssembly.Module` objects cannot be serialized.
  pub compiled_wasm_module_store: Option<CompiledWasmModuleStore>,
  pub stdio: Stdio,
  pub feature_checker: Arc<FeatureChecker>,
}

impl Default for WorkerOptions {
  fn default() -> Self {
    Self {
      create_web_worker_cb: Arc::new(|_| {
        unimplemented!("web workers are not supported")
      }),
      fs: Arc::new(deno_fs::RealFs),
      module_loader: Rc::new(FsModuleLoader),
      skip_op_registration: false,
      seed: None,
      unsafely_ignore_certificate_errors: Default::default(),
      should_break_on_first_statement: Default::default(),
      should_wait_for_inspector_session: Default::default(),
      strace_ops: Default::default(),
      compiled_wasm_module_store: Default::default(),
      shared_array_buffer_store: Default::default(),
      maybe_inspector_server: Default::default(),
      format_js_error_fn: Default::default(),
      get_error_class_fn: Default::default(),
      origin_storage_dir: Default::default(),
      cache_storage_dir: Default::default(),
      broadcast_channel: Default::default(),
      source_map_getter: Default::default(),
      root_cert_store_provider: Default::default(),
      npm_resolver: Default::default(),
      blob_store: Default::default(),
      extensions: Default::default(),
      startup_snapshot: Default::default(),
      create_params: Default::default(),
      bootstrap: Default::default(),
      stdio: Default::default(),
      feature_checker: Default::default(),
    }
  }
}

fn create_op_metrics(
  enable_op_summary_metrics: bool,
  strace_ops: Option<Vec<String>>,
) -> (
  Option<Rc<OpMetricsSummaryTracker>>,
  Option<OpMetricsFactoryFn>,
) {
  let mut op_summary_metrics = None;
  let mut op_metrics_factory_fn: Option<OpMetricsFactoryFn> = None;
  let now = Instant::now();
  let max_len: Rc<std::cell::Cell<usize>> = Default::default();
  if let Some(patterns) = strace_ops {
    /// Match an op name against a list of patterns
    fn matches_pattern(patterns: &[String], name: &str) -> bool {
      let mut found_match = false;
      let mut found_nomatch = false;
      for pattern in patterns.iter() {
        if let Some(pattern) = pattern.strip_prefix('-') {
          if name.contains(pattern) {
            return false;
          }
        } else if name.contains(pattern.as_str()) {
          found_match = true;
        } else {
          found_nomatch = true;
        }
      }

      found_match || !found_nomatch
    }

    op_metrics_factory_fn = Some(Box::new(move |_, _, decl| {
      // If we don't match a requested pattern, or we match a negative pattern, bail
      if !matches_pattern(&patterns, decl.name) {
        return None;
      }

      max_len.set(max_len.get().max(decl.name.len()));
      let max_len = max_len.clone();
      Some(Rc::new(
        move |op: &deno_core::_ops::OpCtx, event, source| {
          eprintln!(
            "[{: >10.3}] {name:max_len$}: {event:?} {source:?}",
            now.elapsed().as_secs_f64(),
            name = op.decl().name,
            max_len = max_len.get()
          );
        },
      ))
    }));
  }

  if enable_op_summary_metrics {
    let summary = Rc::new(OpMetricsSummaryTracker::default());
    let summary_metrics = summary.clone().op_metrics_factory_fn(|_| true);
    op_metrics_factory_fn = Some(match op_metrics_factory_fn {
      Some(f) => merge_op_metrics(f, summary_metrics),
      None => summary_metrics,
    });
    op_summary_metrics = Some(summary);
  }

  (op_summary_metrics, op_metrics_factory_fn)
}

impl MainWorker {
  pub fn bootstrap_from_options(
    main_module: ModuleSpecifier,
    permissions: PermissionsContainer,
    options: WorkerOptions,
  ) -> Self {
    let bootstrap_options = options.bootstrap.clone();
    let mut worker = Self::from_options(main_module, permissions, options);
    worker.bootstrap(bootstrap_options);
    worker
  }

  pub fn from_options(
    main_module: ModuleSpecifier,
    permissions: PermissionsContainer,
    mut options: WorkerOptions,
  ) -> Self {
    deno_core::extension!(deno_permissions_worker,
      options = {
        permissions: PermissionsContainer,
        enable_testing_features: bool,
      },
      state = |state, options| {
        state.put::<PermissionsContainer>(options.permissions);
        state.put(ops::TestingFeaturesEnabled(options.enable_testing_features));
      },
    );

    // Get our op metrics
    let (op_summary_metrics, op_metrics_factory_fn) = create_op_metrics(
      options.bootstrap.enable_op_summary_metrics,
      options.strace_ops,
    );

    // Permissions: many ops depend on this
    let enable_testing_features = options.bootstrap.enable_testing_features;
    let exit_code = ExitCode(Arc::new(AtomicI32::new(0)));
    let create_cache = options.cache_storage_dir.map(|storage_dir| {
      let create_cache_fn = move || SqliteBackedCache::new(storage_dir.clone());
      CreateCache(Arc::new(create_cache_fn))
    });

    // NOTE(bartlomieju): ordering is important here, keep it in sync with
    // `runtime/web_worker.rs` and `runtime/snapshot.rs`!
    let mut extensions = vec![
      // Web APIs
      deno_webidl::deno_webidl::init_ops_and_esm(),
      deno_console::deno_console::init_ops_and_esm(),
      deno_url::deno_url::init_ops_and_esm(),
      deno_web::deno_web::init_ops_and_esm::<PermissionsContainer>(
        options.blob_store.clone(),
        options.bootstrap.location.clone(),
      ),
      deno_webgpu::deno_webgpu::init_ops_and_esm(),
      deno_canvas::deno_canvas::init_ops_and_esm(),
      deno_fetch::deno_fetch::init_ops_and_esm::<PermissionsContainer>(
        deno_fetch::Options {
          user_agent: options.bootstrap.user_agent.clone(),
          root_cert_store_provider: options.root_cert_store_provider.clone(),
          unsafely_ignore_certificate_errors: options
            .unsafely_ignore_certificate_errors
            .clone(),
          file_fetch_handler: Rc::new(deno_fetch::FsFetchHandler),
          ..Default::default()
        },
      ),
      deno_cache::deno_cache::init_ops_and_esm::<SqliteBackedCache>(
        create_cache,
      ),
      deno_websocket::deno_websocket::init_ops_and_esm::<PermissionsContainer>(
        options.bootstrap.user_agent.clone(),
        options.root_cert_store_provider.clone(),
        options.unsafely_ignore_certificate_errors.clone(),
      ),
      deno_webstorage::deno_webstorage::init_ops_and_esm(
        options.origin_storage_dir.clone(),
      ),
      deno_crypto::deno_crypto::init_ops_and_esm(options.seed),
      deno_broadcast_channel::deno_broadcast_channel::init_ops_and_esm(
        options.broadcast_channel.clone(),
      ),
      deno_ffi::deno_ffi::init_ops_and_esm::<PermissionsContainer>(),
      deno_net::deno_net::init_ops_and_esm::<PermissionsContainer>(
        options.root_cert_store_provider.clone(),
        options.unsafely_ignore_certificate_errors.clone(),
      ),
      deno_tls::deno_tls::init_ops_and_esm(),
      deno_kv::deno_kv::init_ops_and_esm(
        MultiBackendDbHandler::remote_or_sqlite::<PermissionsContainer>(
          options.origin_storage_dir.clone(),
          options.seed,
          deno_kv::remote::HttpOptions {
            user_agent: options.bootstrap.user_agent.clone(),
            root_cert_store_provider: options.root_cert_store_provider.clone(),
            unsafely_ignore_certificate_errors: options
              .unsafely_ignore_certificate_errors
              .clone(),
            client_cert_chain_and_key: None,
            proxy: None,
          },
        ),
      ),
      deno_cron::deno_cron::init_ops_and_esm(LocalCronHandler::new()),
      deno_napi::deno_napi::init_ops_and_esm::<PermissionsContainer>(),
      deno_http::deno_http::init_ops_and_esm::<DefaultHttpPropertyExtractor>(),
      deno_io::deno_io::init_ops_and_esm(Some(options.stdio)),
      deno_fs::deno_fs::init_ops_and_esm::<PermissionsContainer>(
        options.fs.clone(),
      ),
      deno_node::deno_node::init_ops_and_esm::<PermissionsContainer>(
        options.npm_resolver,
        options.fs,
      ),
      // Ops from this crate
      ops::runtime::deno_runtime::init_ops_and_esm(main_module.clone()),
      ops::worker_host::deno_worker_host::init_ops_and_esm(
        options.create_web_worker_cb.clone(),
        options.format_js_error_fn.clone(),
      ),
      ops::fs_events::deno_fs_events::init_ops_and_esm(),
      ops::os::deno_os::init_ops_and_esm(exit_code.clone()),
      ops::permissions::deno_permissions::init_ops_and_esm(),
      ops::process::deno_process::init_ops_and_esm(),
      ops::signal::deno_signal::init_ops_and_esm(),
      ops::tty::deno_tty::init_ops_and_esm(),
      ops::http::deno_http_runtime::init_ops_and_esm(),
      ops::bootstrap::deno_bootstrap::init_ops_and_esm(
        if options.startup_snapshot.is_some() {
          None
        } else {
          Some(Default::default())
        },
      ),
      deno_permissions_worker::init_ops_and_esm(
        permissions,
        enable_testing_features,
      ),
      runtime::init_ops_and_esm(),
    ];

    #[cfg(__runtime_js_sources)]
    assert!(cfg!(not(feature = "only_snapshotted_js_sources")), "'__runtime_js_sources' is incompatible with 'only_snapshotted_js_sources'.");

    for extension in &mut extensions {
      if options.startup_snapshot.is_some() {
        extension.js_files = std::borrow::Cow::Borrowed(&[]);
        extension.esm_files = std::borrow::Cow::Borrowed(&[]);
        extension.esm_entry_point = None;
      }
      #[cfg(not(feature = "only_snapshotted_js_sources"))]
      {
        use crate::shared::maybe_transpile_source;
        for source in extension.esm_files.to_mut() {
          maybe_transpile_source(source).unwrap();
        }
        for source in extension.js_files.to_mut() {
          maybe_transpile_source(source).unwrap();
        }
      }
    }

    extensions.extend(std::mem::take(&mut options.extensions));

    #[cfg(feature = "only_snapshotted_js_sources")]
    options.startup_snapshot.as_ref().expect("A user snapshot was not provided, even though 'only_snapshotted_js_sources' is used.");

    let has_notified_of_inspector_disconnect = AtomicBool::new(false);
    let wait_for_inspector_disconnect_callback = Box::new(move || {
      if !has_notified_of_inspector_disconnect
        .swap(true, std::sync::atomic::Ordering::SeqCst)
      {
        println!("Program finished. Waiting for inspector to disconnect to exit the process...");
      }
    });

    let mut js_runtime = JsRuntime::new(RuntimeOptions {
      module_loader: Some(options.module_loader.clone()),
      startup_snapshot: options.startup_snapshot,
      create_params: options.create_params,
      source_map_getter: options.source_map_getter,
      skip_op_registration: options.skip_op_registration,
      get_error_class_fn: options.get_error_class_fn,
      shared_array_buffer_store: options.shared_array_buffer_store.clone(),
      compiled_wasm_module_store: options.compiled_wasm_module_store.clone(),
      extensions,
      inspector: options.maybe_inspector_server.is_some(),
      is_main: true,
      feature_checker: Some(options.feature_checker.clone()),
      op_metrics_factory_fn,
      wait_for_inspector_disconnect_callback: Some(
        wait_for_inspector_disconnect_callback,
      ),
      import_meta_resolve_callback: Some(Box::new(
        import_meta_resolve_callback,
      )),
      validate_import_attributes_cb: Some(Box::new(
        validate_import_attributes_callback,
      )),
      ..Default::default()
    });

    if let Some(op_summary_metrics) = op_summary_metrics {
      js_runtime.op_state().borrow_mut().put(op_summary_metrics);
    }

    if let Some(server) = options.maybe_inspector_server.clone() {
      server.register_inspector(
        main_module.to_string(),
        &mut js_runtime,
        options.should_break_on_first_statement
          || options.should_wait_for_inspector_session,
      );

      // Put inspector handle into the op state so we can put a breakpoint when
      // executing a CJS entrypoint.
      let op_state = js_runtime.op_state();
      let inspector = js_runtime.inspector();
      op_state.borrow_mut().put(inspector);
    }
    let bootstrap_fn_global = {
      let context = js_runtime.main_context();
      let scope = &mut js_runtime.handle_scope();
      let context_local = v8::Local::new(scope, context);
      let global_obj = context_local.global(scope);
      let bootstrap_str =
        v8::String::new_external_onebyte_static(scope, b"bootstrap").unwrap();
      let bootstrap_ns: v8::Local<v8::Object> = global_obj
        .get(scope, bootstrap_str.into())
        .unwrap()
        .try_into()
        .unwrap();
      let main_runtime_str =
        v8::String::new_external_onebyte_static(scope, b"mainRuntime").unwrap();
      let bootstrap_fn =
        bootstrap_ns.get(scope, main_runtime_str.into()).unwrap();
      let bootstrap_fn =
        v8::Local::<v8::Function>::try_from(bootstrap_fn).unwrap();
      v8::Global::new(scope, bootstrap_fn)
    };

    Self {
      js_runtime,
      should_break_on_first_statement: options.should_break_on_first_statement,
      should_wait_for_inspector_session: options
        .should_wait_for_inspector_session,
      exit_code,
      bootstrap_fn_global: Some(bootstrap_fn_global),
    }
  }

  pub fn bootstrap(&mut self, options: BootstrapOptions) {
    // Setup bootstrap options for ops.
    {
      let op_state = self.js_runtime.op_state();
      let mut state = op_state.borrow_mut();
      state.put(options.clone());
      if let Some(node_ipc_fd) = options.node_ipc_fd {
        state.put(deno_node::ChildPipeFd(node_ipc_fd));
      }
    }

    let scope = &mut self.js_runtime.handle_scope();
    let args = options.as_v8(scope);
    let bootstrap_fn = self.bootstrap_fn_global.take().unwrap();
    let bootstrap_fn = v8::Local::new(scope, bootstrap_fn);
    let undefined = v8::undefined(scope);
    bootstrap_fn.call(scope, undefined.into(), &[args]).unwrap();
  }

  /// See [JsRuntime::execute_script](deno_core::JsRuntime::execute_script)
  pub fn execute_script(
    &mut self,
    script_name: &'static str,
    source_code: ModuleCodeString,
  ) -> Result<v8::Global<v8::Value>, AnyError> {
    self.js_runtime.execute_script(script_name, source_code)
  }

  /// Loads and instantiates specified JavaScript module as "main" module.
  pub async fn preload_main_module(
    &mut self,
    module_specifier: &ModuleSpecifier,
  ) -> Result<ModuleId, AnyError> {
    self
      .js_runtime
      .load_main_module(module_specifier, None)
      .await
  }

  /// Loads and instantiates specified JavaScript module as "side" module.
  pub async fn preload_side_module(
    &mut self,
    module_specifier: &ModuleSpecifier,
  ) -> Result<ModuleId, AnyError> {
    self
      .js_runtime
      .load_side_module(module_specifier, None)
      .await
  }

  /// Executes specified JavaScript module.
  pub async fn evaluate_module(
    &mut self,
    id: ModuleId,
  ) -> Result<(), AnyError> {
    self.wait_for_inspector_session();
    let mut receiver = self.js_runtime.mod_evaluate(id);
    tokio::select! {
      // Not using biased mode leads to non-determinism for relatively simple
      // programs.
      biased;

      maybe_result = &mut receiver => {
        debug!("received module evaluate {:#?}", maybe_result);
        maybe_result
      }

      event_loop_result = self.run_event_loop(false) => {
        event_loop_result?;
        receiver.await
      }
    }
  }

  /// Run the event loop up to a given duration. If the runtime resolves early, returns
  /// early. Will always poll the runtime at least once.
  pub async fn run_up_to_duration(
    &mut self,
    duration: Duration,
  ) -> Result<(), AnyError> {
    match tokio::time::timeout(
      duration,
      self
        .js_runtime
        .run_event_loop(PollEventLoopOptions::default()),
    )
    .await
    {
      Ok(Ok(_)) => Ok(()),
      Err(_) => Ok(()),
      Ok(Err(e)) => Err(e),
    }
  }

  /// Loads, instantiates and executes specified JavaScript module.
  pub async fn execute_side_module(
    &mut self,
    module_specifier: &ModuleSpecifier,
  ) -> Result<(), AnyError> {
    let id = self.preload_side_module(module_specifier).await?;
    self.evaluate_module(id).await
  }

  /// Loads, instantiates and executes specified JavaScript module.
  ///
  /// This module will have "import.meta.main" equal to true.
  pub async fn execute_main_module(
    &mut self,
    module_specifier: &ModuleSpecifier,
  ) -> Result<(), AnyError> {
    let id = self.preload_main_module(module_specifier).await?;
    self.evaluate_module(id).await
  }

  fn wait_for_inspector_session(&mut self) {
    if self.should_break_on_first_statement {
      self
        .js_runtime
        .inspector()
        .borrow_mut()
        .wait_for_session_and_break_on_next_statement();
    } else if self.should_wait_for_inspector_session {
      self.js_runtime.inspector().borrow_mut().wait_for_session();
    }
  }

  /// Create new inspector session. This function panics if Worker
  /// was not configured to create inspector.
  pub async fn create_inspector_session(&mut self) -> LocalInspectorSession {
    self.js_runtime.maybe_init_inspector();
    self.js_runtime.inspector().borrow().create_local_session()
  }

  pub async fn run_event_loop(
    &mut self,
    wait_for_inspector: bool,
  ) -> Result<(), AnyError> {
    self
      .js_runtime
      .run_event_loop(deno_core::PollEventLoopOptions {
        wait_for_inspector,
        ..Default::default()
      })
      .await
  }

  /// Return exit code set by the executed code (either in main worker
  /// or one of child web workers).
  pub fn exit_code(&self) -> i32 {
    self.exit_code.get()
  }

  /// Dispatches "load" event to the JavaScript runtime.
  ///
  /// Does not poll event loop, and thus not await any of the "load" event handlers.
  pub fn dispatch_load_event(
    &mut self,
    script_name: &'static str,
  ) -> Result<(), AnyError> {
    self.js_runtime.execute_script(
      script_name,
      // NOTE(@bartlomieju): not using `globalThis` here, because user might delete
      // it. Instead we're using global `dispatchEvent` function which will
      // used a saved reference to global scope.
      ascii_str!("dispatchEvent(new Event('load'))"),
    )?;
    Ok(())
  }

  /// Dispatches "unload" event to the JavaScript runtime.
  ///
  /// Does not poll event loop, and thus not await any of the "unload" event handlers.
  pub fn dispatch_unload_event(
    &mut self,
    script_name: &'static str,
  ) -> Result<(), AnyError> {
    self.js_runtime.execute_script(
      script_name,
      // NOTE(@bartlomieju): not using `globalThis` here, because user might delete
      // it. Instead we're using global `dispatchEvent` function which will
      // used a saved reference to global scope.
      ascii_str!("dispatchEvent(new Event('unload'))"),
    )?;
    Ok(())
  }

  /// Dispatches "beforeunload" event to the JavaScript runtime. Returns a boolean
  /// indicating if the event was prevented and thus event loop should continue
  /// running.
  pub fn dispatch_beforeunload_event(
    &mut self,
    script_name: &'static str,
  ) -> Result<bool, AnyError> {
    let value = self.js_runtime.execute_script(
      script_name,
      // NOTE(@bartlomieju): not using `globalThis` here, because user might delete
      // it. Instead we're using global `dispatchEvent` function which will
      // used a saved reference to global scope.
      ascii_str!(
        "dispatchEvent(new Event('beforeunload', { cancelable: true }));"
      ),
    )?;
    let local_value = value.open(&mut self.js_runtime.handle_scope());
    Ok(local_value.is_false())
  }
}
