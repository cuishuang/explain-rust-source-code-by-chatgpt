// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

// Remove Intl.v8BreakIterator because it is a non-standard API.
delete Intl.v8BreakIterator;

import { core, internals, primordials } from "ext:core/mod.js";
const ops = core.ops;
import {
  op_bootstrap_args,
  op_bootstrap_is_tty,
  op_bootstrap_no_color,
  op_bootstrap_pid,
  op_main_module,
  op_ppid,
  op_set_format_exception_callback,
  op_snapshot_options,
} from "ext:core/ops";
const {
  ArrayPrototypeFilter,
  ArrayPrototypeIncludes,
  ArrayPrototypeMap,
  ArrayPrototypePop,
  ArrayPrototypeShift,
  DateNow,
  Error,
  ErrorPrototype,
  FunctionPrototypeBind,
  FunctionPrototypeCall,
  ObjectAssign,
  ObjectDefineProperties,
  ObjectDefineProperty,
  ObjectKeys,
  ObjectPrototypeIsPrototypeOf,
  ObjectSetPrototypeOf,
  ObjectValues,
  PromisePrototypeThen,
  PromiseResolve,
  SafeSet,
  StringPrototypeIncludes,
  StringPrototypeSplit,
  StringPrototypeTrim,
  Symbol,
  SymbolIterator,
  TypeError,
} = primordials;
const {
  isNativeError,
} = core;
import * as util from "ext:runtime/06_util.js";
import * as event from "ext:deno_web/02_event.js";
import * as location from "ext:deno_web/12_location.js";
import * as version from "ext:runtime/01_version.ts";
import * as os from "ext:runtime/30_os.js";
import * as timers from "ext:deno_web/02_timers.js";
import {
  customInspect,
  getDefaultInspectOptions,
  getNoColor,
  inspectArgs,
  quoteString,
  setNoColorFn,
} from "ext:deno_console/01_console.js";
import * as performance from "ext:deno_web/15_performance.js";
import * as url from "ext:deno_url/00_url.js";
import * as fetch from "ext:deno_fetch/26_fetch.js";
import * as messagePort from "ext:deno_web/13_message_port.js";
import {
  denoNs,
  denoNsUnstable,
  denoNsUnstableById,
  unstableIds,
} from "ext:runtime/90_deno_ns.js";
import { errors } from "ext:runtime/01_errors.js";
import * as webidl from "ext:deno_webidl/00_webidl.js";
import { DOMException } from "ext:deno_web/01_dom_exception.js";
import {
  unstableForWindowOrWorkerGlobalScope,
  windowOrWorkerGlobalScope,
} from "ext:runtime/98_global_scope_shared.js";
import {
  mainRuntimeGlobalProperties,
  memoizeLazy,
} from "ext:runtime/98_global_scope_window.js";
import {
  workerRuntimeGlobalProperties,
} from "ext:runtime/98_global_scope_worker.js";
import { SymbolAsyncDispose, SymbolDispose } from "ext:deno_web/00_infra.js";
// deno-lint-ignore prefer-primordials
if (Symbol.dispose) throw "V8 supports Symbol.dispose now, no need to shim it!";
ObjectDefineProperties(Symbol, {
  dispose: {
    value: SymbolDispose,
    enumerable: false,
    writable: false,
    configurable: false,
  },
  asyncDispose: {
    value: SymbolAsyncDispose,
    enumerable: false,
    writable: false,
    configurable: false,
  },
});

let windowIsClosing = false;
let globalThis_;

let verboseDeprecatedApiWarning = false;
let deprecatedApiWarningDisabled = false;
const ALREADY_WARNED_DEPRECATED = new SafeSet();

function warnOnDeprecatedApi(apiName, stack, ...suggestions) {
  if (deprecatedApiWarningDisabled) {
    return;
  }

  if (!verboseDeprecatedApiWarning) {
    if (ALREADY_WARNED_DEPRECATED.has(apiName)) {
      return;
    }
    ALREADY_WARNED_DEPRECATED.add(apiName);
    console.error(
      `%cwarning: %cUse of deprecated "${apiName}" API. This API will be removed in Deno 2. Run again with DENO_VERBOSE_WARNINGS=1 to get more details.`,
      "color: yellow;",
      "font-weight: bold;",
    );
    return;
  }

  if (ALREADY_WARNED_DEPRECATED.has(apiName + stack)) {
    return;
  }

  // If we haven't warned yet, let's do some processing of the stack trace
  // to make it more useful.
  const stackLines = StringPrototypeSplit(stack, "\n");
  ArrayPrototypeShift(stackLines);
  while (stackLines.length > 0) {
    // Filter out internal frames at the top of the stack - they are not useful
    // to the user.
    if (
      StringPrototypeIncludes(stackLines[0], "(ext:") ||
      StringPrototypeIncludes(stackLines[0], "(node:") ||
      StringPrototypeIncludes(stackLines[0], "<anonymous>")
    ) {
      ArrayPrototypeShift(stackLines);
    } else {
      break;
    }
  }
  // Now remove the last frame if it's coming from "ext:core" - this is most likely
  // event loop tick or promise handler calling a user function - again not
  // useful to the user.
  if (
    stackLines.length > 0 &&
    StringPrototypeIncludes(stackLines[stackLines.length - 1], "(ext:core/")
  ) {
    ArrayPrototypePop(stackLines);
  }

  let isFromRemoteDependency = false;
  const firstStackLine = stackLines[0];
  if (firstStackLine && !StringPrototypeIncludes(firstStackLine, "file:")) {
    isFromRemoteDependency = true;
  }

  ALREADY_WARNED_DEPRECATED.add(apiName + stack);
  console.error(
    `%cwarning: %cUse of deprecated "${apiName}" API. This API will be removed in Deno 2.`,
    "color: yellow;",
    "font-weight: bold;",
  );

  console.error();
  console.error(
    "See the Deno 1 to 2 Migration Guide for more information at https://docs.deno.com/runtime/manual/advanced/migrate_deprecations",
  );
  console.error();
  if (stackLines.length > 0) {
    console.error("Stack trace:");
    for (let i = 0; i < stackLines.length; i++) {
      console.error(`  ${StringPrototypeTrim(stackLines[i])}`);
    }
    console.error();
  }

  for (let i = 0; i < suggestions.length; i++) {
    const suggestion = suggestions[i];
    console.error(
      `%chint: ${suggestion}`,
      "font-weight: bold;",
    );
  }

  if (isFromRemoteDependency) {
    console.error(
      `%chint: It appears this API is used by a remote dependency. Try upgrading to the latest version of that dependency.`,
      "font-weight: bold;",
    );
  }
  console.error();
}

function windowClose() {
  if (!windowIsClosing) {
    windowIsClosing = true;
    // Push a macrotask to exit after a promise resolve.
    // This is not perfect, but should be fine for first pass.
    PromisePrototypeThen(
      PromiseResolve(),
      () =>
        FunctionPrototypeCall(timers.setTimeout, null, () => {
          // This should be fine, since only Window/MainWorker has .close()
          os.exit(0);
        }, 0),
    );
  }
}

function workerClose() {
  if (isClosing) {
    return;
  }

  isClosing = true;
  ops.op_worker_close();
}

function postMessage(message, transferOrOptions = {}) {
  const prefix =
    "Failed to execute 'postMessage' on 'DedicatedWorkerGlobalScope'";
  webidl.requiredArguments(arguments.length, 1, prefix);
  message = webidl.converters.any(message);
  let options;
  if (
    webidl.type(transferOrOptions) === "Object" &&
    transferOrOptions !== undefined &&
    transferOrOptions[SymbolIterator] !== undefined
  ) {
    const transfer = webidl.converters["sequence<object>"](
      transferOrOptions,
      prefix,
      "Argument 2",
    );
    options = { transfer };
  } else {
    options = webidl.converters.StructuredSerializeOptions(
      transferOrOptions,
      prefix,
      "Argument 2",
    );
  }
  const { transfer } = options;
  const data = messagePort.serializeJsMessageData(message, transfer);
  ops.op_worker_post_message(data);
}

let isClosing = false;
let globalDispatchEvent;

async function pollForMessages() {
  const { op_worker_recv_message } = core.ensureFastOps();

  if (!globalDispatchEvent) {
    globalDispatchEvent = FunctionPrototypeBind(
      globalThis.dispatchEvent,
      globalThis,
    );
  }
  while (!isClosing) {
    const data = await op_worker_recv_message();
    if (data === null) break;
    const v = messagePort.deserializeJsMessageData(data);
    const message = v[0];
    const transferables = v[1];

    const msgEvent = new event.MessageEvent("message", {
      cancelable: false,
      data: message,
      ports: ArrayPrototypeFilter(
        transferables,
        (t) =>
          ObjectPrototypeIsPrototypeOf(messagePort.MessagePortPrototype, t),
      ),
    });
    event.setIsTrusted(msgEvent, true);

    try {
      globalDispatchEvent(msgEvent);
    } catch (e) {
      const errorEvent = new event.ErrorEvent("error", {
        cancelable: true,
        message: e.message,
        lineno: e.lineNumber ? e.lineNumber + 1 : undefined,
        colno: e.columnNumber ? e.columnNumber + 1 : undefined,
        filename: e.fileName,
        error: e,
      });

      event.setIsTrusted(errorEvent, true);
      globalDispatchEvent(errorEvent);
      if (!errorEvent.defaultPrevented) {
        throw e;
      }
    }
  }
}

let loadedMainWorkerScript = false;

function importScripts(...urls) {
  if (ops.op_worker_get_type() === "module") {
    throw new TypeError("Can't import scripts in a module worker.");
  }

  const baseUrl = location.getLocationHref();
  const parsedUrls = ArrayPrototypeMap(urls, (scriptUrl) => {
    try {
      return new url.URL(scriptUrl, baseUrl ?? undefined).href;
    } catch {
      throw new DOMException(
        "Failed to parse URL.",
        "SyntaxError",
      );
    }
  });

  // A classic worker's main script has looser MIME type checks than any
  // imported scripts, so we use `loadedMainWorkerScript` to distinguish them.
  // TODO(andreubotella) Refactor worker creation so the main script isn't
  // loaded with `importScripts()`.
  const scripts = ops.op_worker_sync_fetch(
    parsedUrls,
    !loadedMainWorkerScript,
  );
  loadedMainWorkerScript = true;

  for (let i = 0; i < scripts.length; ++i) {
    const { url, script } = scripts[i];
    const err = core.evalContext(script, url)[1];
    if (err !== null) {
      throw err.thrown;
    }
  }
}

const opArgs = memoizeLazy(() => op_bootstrap_args());
const opPid = memoizeLazy(() => op_bootstrap_pid());
const opPpid = memoizeLazy(() => op_ppid());
setNoColorFn(() => op_bootstrap_no_color() || !op_bootstrap_is_tty());

function formatException(error) {
  if (
    isNativeError(error) ||
    ObjectPrototypeIsPrototypeOf(ErrorPrototype, error)
  ) {
    return null;
  } else if (typeof error == "string") {
    return `Uncaught ${
      inspectArgs([quoteString(error, getDefaultInspectOptions())], {
        colors: !getNoColor(),
      })
    }`;
  } else {
    return `Uncaught ${inspectArgs([error], { colors: !getNoColor() })}`;
  }
}

core.registerErrorClass("NotFound", errors.NotFound);
core.registerErrorClass("PermissionDenied", errors.PermissionDenied);
core.registerErrorClass("ConnectionRefused", errors.ConnectionRefused);
core.registerErrorClass("ConnectionReset", errors.ConnectionReset);
core.registerErrorClass("ConnectionAborted", errors.ConnectionAborted);
core.registerErrorClass("NotConnected", errors.NotConnected);
core.registerErrorClass("AddrInUse", errors.AddrInUse);
core.registerErrorClass("AddrNotAvailable", errors.AddrNotAvailable);
core.registerErrorClass("BrokenPipe", errors.BrokenPipe);
core.registerErrorClass("AlreadyExists", errors.AlreadyExists);
core.registerErrorClass("InvalidData", errors.InvalidData);
core.registerErrorClass("TimedOut", errors.TimedOut);
core.registerErrorClass("WouldBlock", errors.WouldBlock);
core.registerErrorClass("WriteZero", errors.WriteZero);
core.registerErrorClass("UnexpectedEof", errors.UnexpectedEof);
core.registerErrorClass("Http", errors.Http);
core.registerErrorClass("Busy", errors.Busy);
core.registerErrorClass("NotSupported", errors.NotSupported);
core.registerErrorClass("FilesystemLoop", errors.FilesystemLoop);
core.registerErrorClass("IsADirectory", errors.IsADirectory);
core.registerErrorClass("NetworkUnreachable", errors.NetworkUnreachable);
core.registerErrorClass("NotADirectory", errors.NotADirectory);
core.registerErrorBuilder(
  "DOMExceptionOperationError",
  function DOMExceptionOperationError(msg) {
    return new DOMException(msg, "OperationError");
  },
);
core.registerErrorBuilder(
  "DOMExceptionQuotaExceededError",
  function DOMExceptionQuotaExceededError(msg) {
    return new DOMException(msg, "QuotaExceededError");
  },
);
core.registerErrorBuilder(
  "DOMExceptionNotSupportedError",
  function DOMExceptionNotSupportedError(msg) {
    return new DOMException(msg, "NotSupported");
  },
);
core.registerErrorBuilder(
  "DOMExceptionNetworkError",
  function DOMExceptionNetworkError(msg) {
    return new DOMException(msg, "NetworkError");
  },
);
core.registerErrorBuilder(
  "DOMExceptionAbortError",
  function DOMExceptionAbortError(msg) {
    return new DOMException(msg, "AbortError");
  },
);
core.registerErrorBuilder(
  "DOMExceptionInvalidCharacterError",
  function DOMExceptionInvalidCharacterError(msg) {
    return new DOMException(msg, "InvalidCharacterError");
  },
);
core.registerErrorBuilder(
  "DOMExceptionDataError",
  function DOMExceptionDataError(msg) {
    return new DOMException(msg, "DataError");
  },
);

function runtimeStart(
  denoVersion,
  v8Version,
  tsVersion,
  target,
) {
  core.setMacrotaskCallback(timers.handleTimerMacrotask);
  core.setWasmStreamingCallback(fetch.handleWasmStreaming);
  core.setReportExceptionCallback(event.reportException);
  op_set_format_exception_callback(formatException);
  version.setVersions(
    denoVersion,
    v8Version,
    tsVersion,
  );
  core.setBuildInfo(target);
}

core.setUnhandledPromiseRejectionHandler(processUnhandledPromiseRejection);
core.setHandledPromiseRejectionHandler(processRejectionHandled);

// Notification that the core received an unhandled promise rejection that is about to
// terminate the runtime. If we can handle it, attempt to do so.
function processUnhandledPromiseRejection(promise, reason) {
  const rejectionEvent = new event.PromiseRejectionEvent(
    "unhandledrejection",
    {
      cancelable: true,
      promise,
      reason,
    },
  );

  // Note that the handler may throw, causing a recursive "error" event
  globalThis_.dispatchEvent(rejectionEvent);

  // If event was not yet prevented, try handing it off to Node compat layer
  // (if it was initialized)
  if (
    !rejectionEvent.defaultPrevented &&
    typeof internals.nodeProcessUnhandledRejectionCallback !== "undefined"
  ) {
    internals.nodeProcessUnhandledRejectionCallback(rejectionEvent);
  }

  // If event was not prevented (or "unhandledrejection" listeners didn't
  // throw) we will let Rust side handle it.
  if (rejectionEvent.defaultPrevented) {
    return true;
  }

  return false;
}

function processRejectionHandled(promise, reason) {
  const rejectionHandledEvent = new event.PromiseRejectionEvent(
    "rejectionhandled",
    { promise, reason },
  );

  // Note that the handler may throw, causing a recursive "error" event
  globalThis_.dispatchEvent(rejectionHandledEvent);

  if (typeof internals.nodeProcessRejectionHandledCallback !== "undefined") {
    internals.nodeProcessRejectionHandledCallback(rejectionHandledEvent);
  }
}

let hasBootstrapped = false;
// Delete the `console` object that V8 automaticaly adds onto the global wrapper
// object on context creation. We don't want this console object to shadow the
// `console` object exposed by the ext/node globalThis proxy.
delete globalThis.console;
// Set up global properties shared by main and worker runtime.
ObjectDefineProperties(globalThis, windowOrWorkerGlobalScope);

// Set up global properties shared by main and worker runtime that are exposed
// by unstable features if those are enabled.
function exposeUnstableFeaturesForWindowOrWorkerGlobalScope(options) {
  const { unstableFlag, unstableFeatures } = options;
  if (unstableFlag) {
    const all = ObjectValues(unstableForWindowOrWorkerGlobalScope);
    for (let i = 0; i <= all.length; i++) {
      const props = all[i];
      ObjectDefineProperties(globalThis, { ...props });
    }
  } else {
    const featureIds = ArrayPrototypeMap(
      ObjectKeys(
        unstableForWindowOrWorkerGlobalScope,
      ),
      (k) => k | 0,
    );

    for (let i = 0; i <= featureIds.length; i++) {
      const featureId = featureIds[i];
      if (ArrayPrototypeIncludes(unstableFeatures, featureId)) {
        const props = unstableForWindowOrWorkerGlobalScope[featureId];
        ObjectDefineProperties(globalThis, { ...props });
      }
    }
  }
}

// NOTE(bartlomieju): remove all the ops that have already been imported using
// "virtual op module" (`ext:core/ops`).
const NOT_IMPORTED_OPS = [
  "op_abort_wasm_streaming",
  "op_add_async",
  "op_add",
  "op_apply_source_map_filename",
  "op_apply_source_map",
  "op_bench_now",
  "op_bootstrap_args",
  "op_bootstrap_is_tty",
  "op_bootstrap_no_color",
  "op_bootstrap_pid",
  "op_broadcast_unsubscribe",
  "op_can_write_vectored",
  "op_close",
  "op_cpus",
  "op_create_brotli_compress",
  "op_create_brotli_decompress",
  "op_current_user_call_site",
  "op_decode",
  "op_deserialize",
  "op_destructure_error",
  "op_dispatch_bench_event",
  "op_dispatch_exception",
  "op_encode_binary_string",
  "op_encode",
  "op_error_async_deferred",
  "op_error_async",
  "op_eval_context",
  "op_event_loop_has_more_work",
  "op_ffi_buf_copy_into",
  "op_ffi_call_nonblocking",
  "op_ffi_call_ptr_nonblocking",
  "op_ffi_call_ptr",
  "op_ffi_cstr_read",
  "op_ffi_get_buf",
  "op_ffi_get_static",
  "op_ffi_load",
  "op_ffi_ptr_create",
  "op_ffi_ptr_equals",
  "op_ffi_ptr_of_exact",
  "op_ffi_ptr_of",
  "op_ffi_ptr_offset",
  "op_ffi_ptr_value",
  "op_ffi_read_bool",
  "op_ffi_read_f32",
  "op_ffi_read_f64",
  "op_ffi_read_i16",
  "op_ffi_read_i32",
  "op_ffi_read_i64",
  "op_ffi_read_i8",
  "op_ffi_read_ptr",
  "op_ffi_read_u16",
  "op_ffi_read_u32",
  "op_ffi_read_u64",
  "op_ffi_read_u8",
  "op_ffi_unsafe_callback_close",
  "op_ffi_unsafe_callback_create",
  "op_ffi_unsafe_callback_ref",
  "op_format_file_name",
  "op_get_promise_details",
  "op_get_proxy_details",
  "op_has_tick_scheduled",
  "op_http_get_request_header",
  "op_http2_accept",
  "op_http2_client_end_stream",
  "op_http2_client_get_response_body_chunk",
  "op_http2_client_get_response_trailers",
  "op_http2_client_get_response",
  "op_http2_client_request",
  "op_http2_client_reset_stream",
  "op_http2_client_send_data",
  "op_http2_client_send_trailers",
  "op_http2_connect",
  "op_http2_listen",
  "op_http2_poll_client_connection",
  "op_http2_send_response",
  "op_image_decode_png",
  "op_image_process",
  "op_is_any_array_buffer",
  "op_is_arguments_object",
  "op_is_array_buffer_view",
  "op_is_array_buffer",
  "op_is_async_function",
  "op_is_big_int_object",
  "op_is_boolean_object",
  "op_is_boxed_primitive",
  "op_is_data_view",
  "op_is_date",
  "op_is_generator_function",
  "op_is_generator_object",
  "op_is_map_iterator",
  "op_is_map",
  "op_is_module_namespace_object",
  "op_is_native_error",
  "op_is_number_object",
  "op_is_promise",
  "op_is_proxy",
  "op_is_reg_exp",
  "op_is_set_iterator",
  "op_is_set",
  "op_is_shared_array_buffer",
  "op_is_string_object",
  "op_is_symbol_object",
  "op_is_typed_array",
  "op_is_weak_map",
  "op_is_weak_set",
  "op_main_module",
  "op_memory_usage",
  "op_napi_open",
  "op_npm_process_state",
  "op_op_names",
  "op_panic",
  "op_pledge_test_permissions",
  "op_ppid",
  "op_print",
  "op_queue_microtask",
  "op_raw_write_vectored",
  "op_read_all",
  "op_read_sync",
  "op_read",
  "op_ref_op",
  "op_register_bench",
  "op_register_test_step",
  "op_register_test",
  "op_resources",
  "op_restore_test_permissions",
  "op_run_microtasks",
  "op_serialize",
  "op_set_exit_code",
  "op_set_format_exception_callback",
  "op_set_handled_promise_rejection_handler",
  "op_set_has_tick_scheduled",
  "op_set_promise_hooks",
  "op_set_wasm_streaming_callback",
  "op_shutdown",
  "op_snapshot_options",
  "op_spawn_child",
  "op_str_byte_length",
  "op_test_event_step_result_failed",
  "op_test_event_step_result_ignored",
  "op_test_event_step_result_ok",
  "op_test_event_step_wait",
  "op_test_op_sanitizer_collect",
  "op_test_op_sanitizer_finish",
  "op_test_op_sanitizer_report",
  "op_timer_cancel",
  "op_timer_queue",
  "op_timer_ref",
  "op_timer_unref",
  "op_try_close",
  "op_unref_op",
  "op_v8_cached_data_version_tag",
  "op_v8_get_heap_statistics",
  "op_vm_run_in_new_context",
  "op_void_async",
  "op_void_sync",
  "op_worker_close",
  "op_worker_get_type",
  "op_worker_post_message",
  "op_worker_recv_message",
  "op_worker_sync_fetch",
  "op_write_all",
  "op_write_sync",
  "op_write_type_error",
  "op_write",
  "op_ws_send_pong",
  "op_jupyter_broadcast",
];

function removeImportedOps() {
  const allOpNames = ObjectKeys(ops);
  for (let i = 0; i < allOpNames.length; i++) {
    const opName = allOpNames[i];
    if (!ArrayPrototypeIncludes(NOT_IMPORTED_OPS, opName)) {
      delete ops[opName];
    }
  }
}

// FIXME(bartlomieju): temporarily add whole `Deno.core` to
// `Deno[Deno.internal]` namespace. It should be removed and only necessary
// methods should be left there.
ObjectAssign(internals, { core, warnOnDeprecatedApi });
const internalSymbol = Symbol("Deno.internal");
const finalDenoNs = {
  internal: internalSymbol,
  [internalSymbol]: internals,
  resources() {
    internals.warnOnDeprecatedApi("Deno.resources()", new Error().stack);
    return core.resources();
  },
  close(rid) {
    internals.warnOnDeprecatedApi(
      "Deno.close()",
      new Error().stack,
      "Use `closer.close()` instead.",
    );
    core.close(rid);
  },
  ...denoNs,
  // Deno.test and Deno.bench are noops here, but kept for compatibility; so
  // that they don't cause errors when used outside of `deno test`/`deno bench`
  // contexts.
  test: () => {},
  bench: () => {},
};

const {
  denoVersion,
  tsVersion,
  v8Version,
  target,
} = op_snapshot_options();

function bootstrapMainRuntime(runtimeOptions) {
  if (hasBootstrapped) {
    throw new Error("Worker runtime already bootstrapped");
  }
  const nodeBootstrap = globalThis.nodeBootstrap;

  const {
    0: location_,
    1: unstableFlag,
    2: unstableFeatures,
    3: inspectFlag,
    5: hasNodeModulesDir,
    6: maybeBinaryNpmCommandName,
    7: shouldDisableDeprecatedApiWarning,
    8: shouldUseVerboseDeprecatedApiWarning,
  } = runtimeOptions;

  removeImportedOps();

  deprecatedApiWarningDisabled = shouldDisableDeprecatedApiWarning;
  verboseDeprecatedApiWarning = shouldUseVerboseDeprecatedApiWarning;
  performance.setTimeOrigin(DateNow());
  globalThis_ = globalThis;

  // Remove bootstrapping data from the global scope
  delete globalThis.__bootstrap;
  delete globalThis.bootstrap;
  delete globalThis.nodeBootstrap;
  hasBootstrapped = true;

  // If the `--location` flag isn't set, make `globalThis.location` `undefined` and
  // writable, so that they can mock it themselves if they like. If the flag was
  // set, define `globalThis.location`, using the provided value.
  if (location_ == null) {
    mainRuntimeGlobalProperties.location = {
      writable: true,
    };
  } else {
    location.setLocationHref(location_);
  }

  exposeUnstableFeaturesForWindowOrWorkerGlobalScope({
    unstableFlag,
    unstableFeatures,
  });
  ObjectDefineProperties(globalThis, mainRuntimeGlobalProperties);
  ObjectDefineProperties(globalThis, {
    // TODO(bartlomieju): in the future we might want to change the
    // behavior of setting `name` to actually update the process name.
    // Empty string matches what browsers do.
    name: util.writable(""),
    close: util.writable(windowClose),
    closed: util.getterOnly(() => windowIsClosing),
  });
  ObjectSetPrototypeOf(globalThis, Window.prototype);

  if (inspectFlag) {
    const consoleFromDeno = globalThis.console;
    core.wrapConsole(consoleFromDeno, core.v8Console);
  }

  event.setEventTargetData(globalThis);
  event.saveGlobalThisReference(globalThis);

  event.defineEventHandler(globalThis, "error");
  event.defineEventHandler(globalThis, "load");
  event.defineEventHandler(globalThis, "beforeunload");
  event.defineEventHandler(globalThis, "unload");
  event.defineEventHandler(globalThis, "unhandledrejection");

  runtimeStart(
    denoVersion,
    v8Version,
    tsVersion,
    target,
  );

  ObjectDefineProperties(finalDenoNs, {
    pid: util.getterOnly(opPid),
    ppid: util.getterOnly(opPpid),
    noColor: util.getterOnly(() => op_bootstrap_no_color()),
    args: util.getterOnly(opArgs),
    mainModule: util.getterOnly(() => op_main_module()),
    // TODO(kt3k): Remove this export at v2
    // See https://github.com/denoland/deno/issues/9294
    customInspect: {
      get() {
        warnOnDeprecatedApi(
          "Deno.customInspect",
          new Error().stack,
          'Use `Symbol.for("Deno.customInspect")` instead.',
        );
        return customInspect;
      },
    },
  });

  // TODO(bartlomieju): deprecate --unstable
  if (unstableFlag) {
    ObjectAssign(finalDenoNs, denoNsUnstable);
    // TODO(bartlomieju): this is not ideal, but because we use `ObjectAssign`
    // above any properties that are defined elsewhere using `Object.defineProperty`
    // are lost.
    let jupyterNs = undefined;
    ObjectDefineProperty(finalDenoNs, "jupyter", {
      get() {
        if (jupyterNs) {
          return jupyterNs;
        }
        throw new Error(
          "Deno.jupyter is only available in `deno jupyter` subcommand.",
        );
      },
      set(val) {
        jupyterNs = val;
      },
    });
  } else {
    for (let i = 0; i <= unstableFeatures.length; i++) {
      const id = unstableFeatures[i];
      ObjectAssign(finalDenoNs, denoNsUnstableById[id]);
    }
  }

  if (!ArrayPrototypeIncludes(unstableFeatures, unstableIds.unsafeProto)) {
    // Removes the `__proto__` for security reasons.
    // https://tc39.es/ecma262/#sec-get-object.prototype.__proto__
    delete Object.prototype.__proto__;
  }

  // Setup `Deno` global - we're actually overriding already existing global
  // `Deno` with `Deno` namespace from "./deno.ts".
  ObjectDefineProperty(globalThis, "Deno", util.readOnly(finalDenoNs));

  if (nodeBootstrap) {
    nodeBootstrap(hasNodeModulesDir, maybeBinaryNpmCommandName);
  }
}

function bootstrapWorkerRuntime(
  runtimeOptions,
  name,
  internalName,
) {
  if (hasBootstrapped) {
    throw new Error("Worker runtime already bootstrapped");
  }

  const nodeBootstrap = globalThis.nodeBootstrap;

  const {
    0: location_,
    1: unstableFlag,
    2: unstableFeatures,
    4: enableTestingFeaturesFlag,
    5: hasNodeModulesDir,
    6: maybeBinaryNpmCommandName,
    7: shouldDisableDeprecatedApiWarning,
    8: shouldUseVerboseDeprecatedApiWarning,
  } = runtimeOptions;

  deprecatedApiWarningDisabled = shouldDisableDeprecatedApiWarning;
  verboseDeprecatedApiWarning = shouldUseVerboseDeprecatedApiWarning;
  performance.setTimeOrigin(DateNow());
  globalThis_ = globalThis;

  removeImportedOps();

  // Remove bootstrapping data from the global scope
  delete globalThis.__bootstrap;
  delete globalThis.bootstrap;
  delete globalThis.nodeBootstrap;
  hasBootstrapped = true;

  exposeUnstableFeaturesForWindowOrWorkerGlobalScope({
    unstableFlag,
    unstableFeatures,
  });
  ObjectDefineProperties(globalThis, workerRuntimeGlobalProperties);
  ObjectDefineProperties(globalThis, {
    name: util.writable(name),
    // TODO(bartlomieju): should be readonly?
    close: util.nonEnumerable(workerClose),
    postMessage: util.writable(postMessage),
  });
  if (enableTestingFeaturesFlag) {
    ObjectDefineProperty(
      globalThis,
      "importScripts",
      util.writable(importScripts),
    );
  }
  ObjectSetPrototypeOf(globalThis, DedicatedWorkerGlobalScope.prototype);

  const consoleFromDeno = globalThis.console;
  core.wrapConsole(consoleFromDeno, core.v8Console);

  event.setEventTargetData(globalThis);
  event.saveGlobalThisReference(globalThis);

  event.defineEventHandler(self, "message");
  event.defineEventHandler(self, "error", undefined, true);
  event.defineEventHandler(self, "unhandledrejection");

  // `Deno.exit()` is an alias to `self.close()`. Setting and exit
  // code using an op in worker context is a no-op.
  os.setExitHandler((_exitCode) => {
    workerClose();
  });

  runtimeStart(
    denoVersion,
    v8Version,
    tsVersion,
    target,
    internalName ?? name,
  );

  location.setLocationHref(location_);

  globalThis.pollForMessages = pollForMessages;

  // TODO(bartlomieju): deprecate --unstable
  if (unstableFlag) {
    ObjectAssign(finalDenoNs, denoNsUnstable);
  } else {
    for (let i = 0; i <= unstableFeatures.length; i++) {
      const id = unstableFeatures[i];
      ObjectAssign(finalDenoNs, denoNsUnstableById[id]);
    }
  }

  if (!ArrayPrototypeIncludes(unstableFeatures, unstableIds.unsafeProto)) {
    // Removes the `__proto__` for security reasons.
    // https://tc39.es/ecma262/#sec-get-object.prototype.__proto__
    delete Object.prototype.__proto__;
  }

  ObjectDefineProperties(finalDenoNs, {
    pid: util.getterOnly(opPid),
    noColor: util.getterOnly(() => op_bootstrap_no_color()),
    args: util.getterOnly(opArgs),
    // TODO(kt3k): Remove this export at v2
    // See https://github.com/denoland/deno/issues/9294
    customInspect: {
      get() {
        warnOnDeprecatedApi(
          "Deno.customInspect",
          new Error().stack,
          'Use `Symbol.for("Deno.customInspect")` instead.',
        );
        return customInspect;
      },
    },
  });
  // Setup `Deno` global - we're actually overriding already
  // existing global `Deno` with `Deno` namespace from "./deno.ts".
  ObjectDefineProperty(globalThis, "Deno", util.readOnly(finalDenoNs));

  if (nodeBootstrap) {
    nodeBootstrap(hasNodeModulesDir, maybeBinaryNpmCommandName);
  }
}

globalThis.bootstrap = {
  mainRuntime: bootstrapMainRuntime,
  workerRuntime: bootstrapWorkerRuntime,
};
