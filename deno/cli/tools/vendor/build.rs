// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use std::fmt::Write as _;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use deno_ast::ModuleSpecifier;
use deno_core::anyhow::bail;
use deno_core::anyhow::Context;
use deno_core::error::AnyError;
use deno_core::futures::future::LocalBoxFuture;
use deno_core::parking_lot::Mutex;
use deno_graph::source::ResolutionMode;
use deno_graph::JsModule;
use deno_graph::Module;
use deno_graph::ModuleGraph;
use deno_runtime::deno_fs;
use import_map::ImportMap;
use import_map::SpecifierMap;

use crate::args::JsxImportSourceConfig;
use crate::args::Lockfile;
use crate::cache::ParsedSourceCache;
use crate::graph_util;
use crate::graph_util::graph_lock_or_exit;
use crate::tools::vendor::import_map::BuildImportMapInput;

use super::analyze::has_default_export;
use super::import_map::build_import_map;
use super::mappings::Mappings;
use super::mappings::ProxiedModule;
use super::specifiers::is_remote_specifier;

/// Allows substituting the environment for testing purposes.
pub trait VendorEnvironment {
  fn cwd(&self) -> Result<PathBuf, AnyError>;
  fn create_dir_all(&self, dir_path: &Path) -> Result<(), AnyError>;
  fn write_file(&self, file_path: &Path, bytes: &[u8]) -> Result<(), AnyError>;
  fn path_exists(&self, path: &Path) -> bool;
}

pub struct RealVendorEnvironment;

impl VendorEnvironment for RealVendorEnvironment {
  fn cwd(&self) -> Result<PathBuf, AnyError> {
    Ok(std::env::current_dir()?)
  }

  fn create_dir_all(&self, dir_path: &Path) -> Result<(), AnyError> {
    Ok(std::fs::create_dir_all(dir_path)?)
  }

  fn write_file(&self, file_path: &Path, bytes: &[u8]) -> Result<(), AnyError> {
    std::fs::write(file_path, bytes)
      .with_context(|| format!("Failed writing {}", file_path.display()))
  }

  fn path_exists(&self, path: &Path) -> bool {
    path.exists()
  }
}

type BuildGraphFuture = LocalBoxFuture<'static, Result<ModuleGraph, AnyError>>;

pub struct BuildInput<
  'a,
  TBuildGraphFn: FnOnce(Vec<ModuleSpecifier>) -> BuildGraphFuture,
  TEnvironment: VendorEnvironment,
> {
  pub entry_points: Vec<ModuleSpecifier>,
  pub build_graph: TBuildGraphFn,
  pub parsed_source_cache: &'a ParsedSourceCache,
  pub output_dir: &'a Path,
  pub maybe_original_import_map: Option<&'a ImportMap>,
  pub maybe_lockfile: Option<Arc<Mutex<Lockfile>>>,
  pub maybe_jsx_import_source: Option<&'a JsxImportSourceConfig>,
  pub resolver: &'a dyn deno_graph::source::Resolver,
  pub environment: &'a TEnvironment,
}

pub struct BuildOutput {
  pub vendored_count: usize,
  pub graph: ModuleGraph,
}

/// Vendors remote modules and returns how many were vendored.
pub async fn build<
  TBuildGraphFn: FnOnce(Vec<ModuleSpecifier>) -> BuildGraphFuture,
  TEnvironment: VendorEnvironment,
>(
  input: BuildInput<'_, TBuildGraphFn, TEnvironment>,
) -> Result<BuildOutput, AnyError> {
  let BuildInput {
    mut entry_points,
    build_graph,
    parsed_source_cache,
    output_dir,
    maybe_original_import_map: original_import_map,
    maybe_lockfile,
    maybe_jsx_import_source: jsx_import_source,
    resolver,
    environment,
  } = input;
  assert!(output_dir.is_absolute());
  let output_dir_specifier =
    ModuleSpecifier::from_directory_path(output_dir).unwrap();

  if let Some(original_im) = &original_import_map {
    validate_original_import_map(original_im, &output_dir_specifier)?;
  }

  // add the jsx import source to the entry points to ensure it is always vendored
  if let Some(jsx_import_source) = jsx_import_source {
    if let Some(specifier_text) = jsx_import_source.maybe_specifier_text() {
      if let Ok(specifier) = resolver.resolve(
        &specifier_text,
        &deno_graph::Range {
          specifier: jsx_import_source.base_url.clone(),
          start: deno_graph::Position::zeroed(),
          end: deno_graph::Position::zeroed(),
        },
        ResolutionMode::Execution,
      ) {
        entry_points.push(specifier);
      }
    }
  }

  let graph = build_graph(entry_points).await?;

  // check the lockfile
  if let Some(lockfile) = maybe_lockfile {
    graph_lock_or_exit(&graph, &mut lockfile.lock());
  }

  // surface any errors
  graph_util::graph_valid(
    &graph,
    &deno_fs::RealFs,
    &graph.roots,
    graph_util::GraphValidOptions {
      is_vendoring: true,
      check_js: true,
      follow_type_only: true,
    },
  )?;

  // figure out how to map remote modules to local
  let all_modules = graph.modules().collect::<Vec<_>>();
  let remote_modules = all_modules
    .iter()
    .filter(|m| is_remote_specifier(m.specifier()))
    .copied()
    .collect::<Vec<_>>();
  let mappings =
    Mappings::from_remote_modules(&graph, &remote_modules, output_dir)?;

  // write out all the files
  for module in &remote_modules {
    let source = match module {
      Module::Js(module) => &module.source,
      Module::Json(module) => &module.source,
      Module::Node(_) | Module::Npm(_) | Module::External(_) => continue,
    };
    let specifier = module.specifier();
    let local_path = mappings
      .proxied_path(specifier)
      .unwrap_or_else(|| mappings.local_path(specifier));

    environment.create_dir_all(local_path.parent().unwrap())?;
    environment.write_file(&local_path, source.as_bytes())?;
  }

  // write out the proxies
  for (specifier, proxied_module) in mappings.proxied_modules() {
    let proxy_path = mappings.local_path(specifier);
    let module = graph.get(specifier).unwrap().js().unwrap();
    let text =
      build_proxy_module_source(module, proxied_module, parsed_source_cache)?;

    environment.write_file(&proxy_path, text.as_bytes())?;
  }

  // create the import map if necessary
  if !remote_modules.is_empty() {
    let import_map_path = output_dir.join("import_map.json");
    let import_map_text = build_import_map(BuildImportMapInput {
      base_dir: &output_dir_specifier,
      graph: &graph,
      modules: &all_modules,
      mappings: &mappings,
      original_import_map,
      jsx_import_source,
      resolver,
      parsed_source_cache,
    })?;
    environment.write_file(&import_map_path, import_map_text.as_bytes())?;
  }

  Ok(BuildOutput {
    vendored_count: remote_modules.len(),
    graph,
  })
}

fn validate_original_import_map(
  import_map: &ImportMap,
  output_dir: &ModuleSpecifier,
) -> Result<(), AnyError> {
  fn validate_imports(
    imports: &SpecifierMap,
    output_dir: &ModuleSpecifier,
  ) -> Result<(), AnyError> {
    for entry in imports.entries() {
      if let Some(value) = entry.value {
        if value.as_str().starts_with(output_dir.as_str()) {
          bail!(
            "Providing an existing import map with entries for the output directory is not supported (\"{}\": \"{}\").",
            entry.raw_key,
            entry.raw_value.unwrap_or("<INVALID>"),
          );
        }
      }
    }
    Ok(())
  }

  validate_imports(import_map.imports(), output_dir)?;

  for scope in import_map.scopes() {
    if scope.key.starts_with(output_dir.as_str()) {
      bail!(
        "Providing an existing import map with a scope for the output directory is not supported (\"{}\").",
        scope.raw_key,
      );
    }
    validate_imports(scope.imports, output_dir)?;
  }

  Ok(())
}

fn build_proxy_module_source(
  module: &JsModule,
  proxied_module: &ProxiedModule,
  parsed_source_cache: &ParsedSourceCache,
) -> Result<String, AnyError> {
  let mut text = String::new();
  writeln!(
    text,
    "// @deno-types=\"{}\"",
    proxied_module.declaration_specifier
  )
  .unwrap();

  let relative_specifier = format!(
    "./{}",
    proxied_module
      .output_path
      .file_name()
      .unwrap()
      .to_string_lossy()
  );

  // for simplicity, always include the `export *` statement as it won't error
  // even when the module does not contain a named export
  writeln!(text, "export * from \"{relative_specifier}\";").unwrap();

  // add a default export if one exists in the module
  let parsed_source =
    parsed_source_cache.get_parsed_source_from_js_module(module)?;
  if has_default_export(&parsed_source) {
    writeln!(text, "export {{ default }} from \"{relative_specifier}\";")
      .unwrap();
  }

  Ok(text)
}

#[cfg(test)]
mod test {
  use crate::args::JsxImportSourceConfig;
  use crate::tools::vendor::test::VendorTestBuilder;
  use deno_core::serde_json::json;
  use pretty_assertions::assert_eq;

  #[tokio::test]
  async fn no_remote_modules() {
    let mut builder = VendorTestBuilder::with_default_setup();
    let output = builder
      .with_loader(|loader| {
        loader.add("/mod.ts", "");
      })
      .build()
      .await
      .unwrap();

    assert_eq!(output.import_map, None,);
    assert_eq!(output.files, vec![],);
  }

  #[tokio::test]
  async fn local_specifiers_to_remote() {
    let mut builder = VendorTestBuilder::with_default_setup();
    let output = builder
      .with_loader(|loader| {
        loader
          .add(
            "/mod.ts",
            concat!(
              r#"import "https://localhost/mod.ts";"#,
              r#"import "https://localhost/other.ts?test";"#,
              r#"import "https://localhost/redirect.ts";"#,
            ),
          )
          .add("https://localhost/mod.ts", "export class Mod {}")
          .add("https://localhost/other.ts?test", "export class Other {}")
          .add_redirect(
            "https://localhost/redirect.ts",
            "https://localhost/mod.ts",
          );
      })
      .build()
      .await
      .unwrap();

    assert_eq!(
      output.import_map,
      Some(json!({
        "imports": {
          "https://localhost/other.ts?test": "./localhost/other.ts",
          "https://localhost/redirect.ts": "./localhost/mod.ts",
          "https://localhost/": "./localhost/",
        }
      }))
    );
    assert_eq!(
      output.files,
      to_file_vec(&[
        ("/vendor/localhost/mod.ts", "export class Mod {}"),
        ("/vendor/localhost/other.ts", "export class Other {}"),
      ]),
    );
  }

  #[tokio::test]
  async fn remote_specifiers() {
    let mut builder = VendorTestBuilder::with_default_setup();
    let output = builder
      .with_loader(|loader| {
        loader
          .add(
            "/mod.ts",
            concat!(
              r#"import "https://localhost/mod.ts";"#,
              r#"import "https://other/mod.ts";"#,
            ),
          )
          .add(
            "https://localhost/mod.ts",
            concat!(
              "export * from './other.ts';",
              "export * from './redirect.ts';",
              "export * from '/absolute.ts';",
            ),
          )
          .add("https://localhost/other.ts", "export class Other {}")
          .add_redirect(
            "https://localhost/redirect.ts",
            "https://localhost/other.ts",
          )
          .add("https://localhost/absolute.ts", "export class Absolute {}")
          .add("https://other/mod.ts", "export * from './sub/mod.ts';")
          .add(
            "https://other/sub/mod.ts",
            concat!(
              "export * from '../sub2/mod.ts';",
              "export * from '../sub2/other?asdf';",
              // reference a path on a different origin
              "export * from 'https://localhost/other.ts';",
              "export * from 'https://localhost/redirect.ts';",
            ),
          )
          .add("https://other/sub2/mod.ts", "export class Mod {}")
          .add_with_headers(
            "https://other/sub2/other?asdf",
            "export class Other {}",
            &[("content-type", "application/javascript")],
          );
      })
      .build()
      .await
      .unwrap();

    assert_eq!(
      output.import_map,
      Some(json!({
        "imports": {
          "https://localhost/": "./localhost/",
          "https://localhost/redirect.ts": "./localhost/other.ts",
          "https://other/": "./other/",
        },
        "scopes": {
          "./localhost/": {
            "./localhost/redirect.ts": "./localhost/other.ts",
            "/absolute.ts": "./localhost/absolute.ts",
          },
          "./other/": {
            "./other/sub2/other?asdf": "./other/sub2/other.js"
          }
        }
      }))
    );
    assert_eq!(
      output.files,
      to_file_vec(&[
        ("/vendor/localhost/absolute.ts", "export class Absolute {}"),
        (
          "/vendor/localhost/mod.ts",
          concat!(
            "export * from './other.ts';",
            "export * from './redirect.ts';",
            "export * from '/absolute.ts';",
          )
        ),
        ("/vendor/localhost/other.ts", "export class Other {}"),
        ("/vendor/other/mod.ts", "export * from './sub/mod.ts';"),
        (
          "/vendor/other/sub/mod.ts",
          concat!(
            "export * from '../sub2/mod.ts';",
            "export * from '../sub2/other?asdf';",
            "export * from 'https://localhost/other.ts';",
            "export * from 'https://localhost/redirect.ts';",
          )
        ),
        ("/vendor/other/sub2/mod.ts", "export class Mod {}"),
        ("/vendor/other/sub2/other.js", "export class Other {}"),
      ]),
    );
  }

  #[tokio::test]
  async fn remote_redirect_entrypoint() {
    let mut builder = VendorTestBuilder::with_default_setup();
    let output = builder
      .with_loader(|loader| {
        loader
          .add(
            "/mod.ts",
            concat!(
              "import * as test from 'https://x.nest.land/Yenv@1.0.0/mod.ts';\n",
              "console.log(test)",
            ),
          )
          .add_redirect("https://x.nest.land/Yenv@1.0.0/mod.ts", "https://arweave.net/VFtWNW3QZ-7__v7c7kck22eFI24OuK1DFzyQHKoZ9AE/mod.ts")
          .add(
            "https://arweave.net/VFtWNW3QZ-7__v7c7kck22eFI24OuK1DFzyQHKoZ9AE/mod.ts",
            "export * from './src/mod.ts'",
          )
          .add(
            "https://arweave.net/VFtWNW3QZ-7__v7c7kck22eFI24OuK1DFzyQHKoZ9AE/src/mod.ts",
            "export class Test {}",
          );
      })
      .build()
      .await
      .unwrap();

    assert_eq!(
      output.import_map,
      Some(json!({
        "imports": {
          "https://x.nest.land/Yenv@1.0.0/mod.ts": "./arweave.net/VFtWNW3QZ-7__v7c7kck22eFI24OuK1DFzyQHKoZ9AE/mod.ts",
          "https://arweave.net/": "./arweave.net/"
        },
      }))
    );
    assert_eq!(
      output.files,
      to_file_vec(&[
        ("/vendor/arweave.net/VFtWNW3QZ-7__v7c7kck22eFI24OuK1DFzyQHKoZ9AE/mod.ts", "export * from './src/mod.ts'"),
        (
          "/vendor/arweave.net/VFtWNW3QZ-7__v7c7kck22eFI24OuK1DFzyQHKoZ9AE/src/mod.ts",
          "export class Test {}",
        ),
      ]),
    );
  }

  #[tokio::test]
  async fn same_target_filename_specifiers() {
    let mut builder = VendorTestBuilder::with_default_setup();
    let output = builder
      .with_loader(|loader| {
        loader
          .add(
            "/mod.ts",
            concat!(
              r#"import "https://localhost/MOD.TS";"#,
              r#"import "https://localhost/mod.TS";"#,
              r#"import "https://localhost/mod.ts";"#,
              r#"import "https://localhost/mod.ts?test";"#,
              r#"import "https://localhost/CAPS.TS";"#,
            ),
          )
          .add("https://localhost/MOD.TS", "export class Mod {}")
          .add("https://localhost/mod.TS", "export class Mod2 {}")
          .add("https://localhost/mod.ts", "export class Mod3 {}")
          .add("https://localhost/mod.ts?test", "export class Mod4 {}")
          .add("https://localhost/CAPS.TS", "export class Caps {}");
      })
      .build()
      .await
      .unwrap();

    assert_eq!(
      output.import_map,
      Some(json!({
        "imports": {
          "https://localhost/mod.TS": "./localhost/mod_2.TS",
          "https://localhost/mod.ts": "./localhost/mod_3.ts",
          "https://localhost/mod.ts?test": "./localhost/mod_4.ts",
          "https://localhost/": "./localhost/",
        }
      }))
    );
    assert_eq!(
      output.files,
      to_file_vec(&[
        ("/vendor/localhost/CAPS.TS", "export class Caps {}"),
        ("/vendor/localhost/MOD.TS", "export class Mod {}"),
        ("/vendor/localhost/mod_2.TS", "export class Mod2 {}"),
        ("/vendor/localhost/mod_3.ts", "export class Mod3 {}"),
        ("/vendor/localhost/mod_4.ts", "export class Mod4 {}"),
      ]),
    );
  }

  #[tokio::test]
  async fn multiple_entrypoints() {
    let mut builder = VendorTestBuilder::with_default_setup();
    let output = builder
      .add_entry_point("/test.deps.ts")
      .with_loader(|loader| {
        loader
          .add("/mod.ts", r#"import "https://localhost/mod.ts";"#)
          .add(
            "/test.deps.ts",
            r#"export * from "https://localhost/test.ts";"#,
          )
          .add("https://localhost/mod.ts", "export class Mod {}")
          .add("https://localhost/test.ts", "export class Test {}");
      })
      .build()
      .await
      .unwrap();

    assert_eq!(
      output.import_map,
      Some(json!({
        "imports": {
          "https://localhost/": "./localhost/",
        }
      }))
    );
    assert_eq!(
      output.files,
      to_file_vec(&[
        ("/vendor/localhost/mod.ts", "export class Mod {}"),
        ("/vendor/localhost/test.ts", "export class Test {}"),
      ]),
    );
  }

  #[tokio::test]
  async fn json_module() {
    let mut builder = VendorTestBuilder::with_default_setup();
    let output = builder
      .with_loader(|loader| {
        loader
          .add(
            "/mod.ts",
            r#"import data from "https://localhost/data.json" assert { type: "json" };"#,
          )
          .add("https://localhost/data.json", "{ \"a\": \"b\" }");
      })
      .build()
      .await
      .unwrap();

    assert_eq!(
      output.import_map,
      Some(json!({
        "imports": {
          "https://localhost/": "./localhost/"
        }
      }))
    );
    assert_eq!(
      output.files,
      to_file_vec(&[("/vendor/localhost/data.json", "{ \"a\": \"b\" }"),]),
    );
  }

  #[tokio::test]
  async fn data_urls() {
    let mut builder = VendorTestBuilder::with_default_setup();

    let mod_file_text = r#"import * as b from "data:application/typescript,export%20*%20from%20%22https://localhost/mod.ts%22;";"#;

    let output = builder
      .with_loader(|loader| {
        loader
          .add("/mod.ts", mod_file_text)
          .add("https://localhost/mod.ts", "export class Example {}");
      })
      .build()
      .await
      .unwrap();

    assert_eq!(
      output.import_map,
      Some(json!({
        "imports": {
          "https://localhost/": "./localhost/"
        }
      }))
    );
    assert_eq!(
      output.files,
      to_file_vec(&[("/vendor/localhost/mod.ts", "export class Example {}"),]),
    );
  }

  #[tokio::test]
  async fn x_typescript_types_no_default() {
    let mut builder = VendorTestBuilder::with_default_setup();
    let output = builder
      .with_loader(|loader| {
        loader
          .add("/mod.ts", r#"import "https://localhost/mod.js";"#)
          .add_with_headers(
            "https://localhost/mod.js",
            "export class Mod {}",
            &[("x-typescript-types", "https://localhost/mod.d.ts")],
          )
          .add("https://localhost/mod.d.ts", "export class Mod {}");
      })
      .build()
      .await
      .unwrap();

    assert_eq!(
      output.import_map,
      Some(json!({
        "imports": {
          "https://localhost/": "./localhost/"
        }
      }))
    );
    assert_eq!(
      output.files,
      to_file_vec(&[
        ("/vendor/localhost/mod.d.ts", "export class Mod {}"),
        (
          "/vendor/localhost/mod.js",
          concat!(
            "// @deno-types=\"https://localhost/mod.d.ts\"\n",
            "export * from \"./mod.proxied.js\";\n"
          )
        ),
        ("/vendor/localhost/mod.proxied.js", "export class Mod {}"),
      ]),
    );
  }

  #[tokio::test]
  async fn x_typescript_types_default_export() {
    let mut builder = VendorTestBuilder::with_default_setup();
    let output = builder
      .with_loader(|loader| {
        loader
          .add("/mod.ts", r#"import "https://localhost/mod.js";"#)
          .add_with_headers(
            "https://localhost/mod.js",
            "export default class Mod {}",
            &[("x-typescript-types", "https://localhost/mod.d.ts")],
          )
          .add("https://localhost/mod.d.ts", "export default class Mod {}");
      })
      .build()
      .await
      .unwrap();

    assert_eq!(
      output.import_map,
      Some(json!({
        "imports": {
          "https://localhost/": "./localhost/"
        }
      }))
    );
    assert_eq!(
      output.files,
      to_file_vec(&[
        ("/vendor/localhost/mod.d.ts", "export default class Mod {}"),
        (
          "/vendor/localhost/mod.js",
          concat!(
            "// @deno-types=\"https://localhost/mod.d.ts\"\n",
            "export * from \"./mod.proxied.js\";\n",
            "export { default } from \"./mod.proxied.js\";\n",
          )
        ),
        (
          "/vendor/localhost/mod.proxied.js",
          "export default class Mod {}"
        ),
      ]),
    );
  }

  #[tokio::test]
  async fn subdir() {
    let mut builder = VendorTestBuilder::with_default_setup();
    let output = builder
      .with_loader(|loader| {
        loader
          .add(
            "/mod.ts",
            r#"import "http://localhost:4545/sub/logger/mod.ts?testing";"#,
          )
          .add(
            "http://localhost:4545/sub/logger/mod.ts?testing",
            "export * from './logger.ts?test';",
          )
          .add(
            "http://localhost:4545/sub/logger/logger.ts?test",
            "export class Logger {}",
          );
      })
      .build()
      .await
      .unwrap();

    assert_eq!(
      output.import_map,
      Some(json!({
        "imports": {
          "http://localhost:4545/sub/logger/mod.ts?testing": "./localhost_4545/sub/logger/mod.ts",
          "http://localhost:4545/": "./localhost_4545/",
        },
        "scopes": {
          "./localhost_4545/": {
            "./localhost_4545/sub/logger/logger.ts?test": "./localhost_4545/sub/logger/logger.ts"
          }
        }
      }))
    );
    assert_eq!(
      output.files,
      to_file_vec(&[
        (
          "/vendor/localhost_4545/sub/logger/logger.ts",
          "export class Logger {}",
        ),
        (
          "/vendor/localhost_4545/sub/logger/mod.ts",
          "export * from './logger.ts?test';"
        ),
      ]),
    );
  }

  #[tokio::test]
  async fn same_origin_absolute_with_redirect() {
    let mut builder = VendorTestBuilder::with_default_setup();
    let output = builder
      .with_loader(|loader| {
        loader
          .add(
            "/mod.ts",
            r#"import "https://localhost/subdir/sub/mod.ts";"#,
          )
          .add(
            "https://localhost/subdir/sub/mod.ts",
            "import 'https://localhost/std/hash/mod.ts'",
          )
          .add_redirect(
            "https://localhost/std/hash/mod.ts",
            "https://localhost/std@0.1.0/hash/mod.ts",
          )
          .add(
            "https://localhost/std@0.1.0/hash/mod.ts",
            "export class Test {}",
          );
      })
      .build()
      .await
      .unwrap();

    assert_eq!(
      output.import_map,
      Some(json!({
        "imports": {
          "https://localhost/std/hash/mod.ts": "./localhost/std@0.1.0/hash/mod.ts",
          "https://localhost/": "./localhost/",
        },
      }))
    );
    assert_eq!(
      output.files,
      to_file_vec(&[
        (
          "/vendor/localhost/std@0.1.0/hash/mod.ts",
          "export class Test {}"
        ),
        (
          "/vendor/localhost/subdir/sub/mod.ts",
          "import 'https://localhost/std/hash/mod.ts'"
        ),
      ]),
    );
  }

  #[tokio::test]
  async fn remote_relative_specifier_with_scheme_like_folder_name() {
    let mut builder = VendorTestBuilder::with_default_setup();
    let output = builder
      .with_loader(|loader| {
        loader
          .add("/mod.ts", "import 'https://localhost/mod.ts';")
          .add(
            "https://localhost/mod.ts",
            "import './npm:test@1.0.0/test/test!cjs?test';import './npm:test@1.0.0/mod.ts';",
          )
          .add(
            "https://localhost/npm:test@1.0.0/mod.ts",
            "console.log(4);",
          )
          .add_with_headers(
            "https://localhost/npm:test@1.0.0/test/test!cjs?test",
            "console.log(5);",
            &[("content-type", "application/javascript")],
          );
      })
      .build()
      .await
      .unwrap();

    assert_eq!(
      output.import_map,
      Some(json!({
        "imports": {
          "https://localhost/": "./localhost/"
        },
        "scopes": {
          "./localhost/": {
            "./localhost/npm:test@1.0.0/mod.ts": "./localhost/npm_test@1.0.0/mod.ts",
            "./localhost/npm:test@1.0.0/test/test!cjs?test": "./localhost/npm_test@1.0.0/test/test!cjs.js",
            "./localhost/npm_test@1.0.0/test/test!cjs?test": "./localhost/npm_test@1.0.0/test/test!cjs.js"
          }
        }
      }))
    );
    assert_eq!(
      output.files,
      to_file_vec(&[
        (
          "/vendor/localhost/mod.ts",
          "import './npm:test@1.0.0/test/test!cjs?test';import './npm:test@1.0.0/mod.ts';"
        ),
        ("/vendor/localhost/npm_test@1.0.0/mod.ts", "console.log(4);"),
        (
          "/vendor/localhost/npm_test@1.0.0/test/test!cjs.js",
          "console.log(5);"
        ),
      ]),
    );
  }

  #[tokio::test]
  async fn existing_import_map_basic() {
    let mut builder = VendorTestBuilder::with_default_setup();
    let mut original_import_map = builder.new_import_map("/import_map2.json");
    original_import_map
      .imports_mut()
      .append(
        "https://localhost/mod.ts".to_string(),
        "./local_vendor/mod.ts".to_string(),
      )
      .unwrap();
    let local_vendor_scope = original_import_map
      .get_or_append_scope_mut("./local_vendor/")
      .unwrap();
    local_vendor_scope
      .append(
        "https://localhost/logger.ts".to_string(),
        "./local_vendor/logger.ts".to_string(),
      )
      .unwrap();
    local_vendor_scope
      .append(
        "/console_logger.ts".to_string(),
        "./local_vendor/console_logger.ts".to_string(),
      )
      .unwrap();

    let output = builder
      .with_loader(|loader| {
        loader.add("/mod.ts", "import 'https://localhost/mod.ts'; import 'https://localhost/other.ts';");
        loader.add("/local_vendor/mod.ts", "import 'https://localhost/logger.ts'; import '/console_logger.ts'; console.log(5);");
        loader.add("/local_vendor/logger.ts", "export class Logger {}");
        loader.add("/local_vendor/console_logger.ts", "export class ConsoleLogger {}");
        loader.add("https://localhost/mod.ts", "console.log(6);");
        loader.add("https://localhost/other.ts", "import './mod.ts';");
      })
      .set_original_import_map(original_import_map)
      .build()
      .await
      .unwrap();

    assert_eq!(
      output.import_map,
      Some(json!({
        "imports": {
          "https://localhost/mod.ts": "../local_vendor/mod.ts",
          "https://localhost/": "./localhost/"
        },
        "scopes": {
          "../local_vendor/": {
            "https://localhost/logger.ts": "../local_vendor/logger.ts",
            "/console_logger.ts": "../local_vendor/console_logger.ts",
          },
          "./localhost/": {
            "./localhost/mod.ts": "../local_vendor/mod.ts",
          },
        }
      }))
    );
    assert_eq!(
      output.files,
      to_file_vec(&[("/vendor/localhost/other.ts", "import './mod.ts';")]),
    );
  }

  #[tokio::test]
  async fn existing_import_map_remote_dep_bare_specifier() {
    let mut builder = VendorTestBuilder::with_default_setup();
    let mut original_import_map = builder.new_import_map("/import_map2.json");
    original_import_map
      .imports_mut()
      .append(
        "twind".to_string(),
        "https://localhost/twind.ts".to_string(),
      )
      .unwrap();

    let output = builder
      .with_loader(|loader| {
        loader.add("/mod.ts", "import 'https://remote/mod.ts';");
        loader.add("https://remote/mod.ts", "import 'twind';");
        loader.add("https://localhost/twind.ts", "export class Test {}");
      })
      .set_original_import_map(original_import_map)
      .build()
      .await
      .unwrap();

    assert_eq!(
      output.import_map,
      Some(json!({
        "imports": {
          "https://localhost/": "./localhost/",
          "https://remote/": "./remote/"
        },
        "scopes": {
          "./remote/": {
            "twind": "./localhost/twind.ts"
          },
        }
      }))
    );
    assert_eq!(
      output.files,
      to_file_vec(&[
        ("/vendor/localhost/twind.ts", "export class Test {}"),
        ("/vendor/remote/mod.ts", "import 'twind';"),
      ]),
    );
  }

  #[tokio::test]
  async fn existing_import_map_mapped_bare_specifier() {
    let mut builder = VendorTestBuilder::with_default_setup();
    let mut original_import_map = builder.new_import_map("/import_map.json");
    let imports = original_import_map.imports_mut();
    imports
      .append("$fresh".to_string(), "https://localhost/fresh".to_string())
      .unwrap();
    imports
      .append("std/".to_string(), "https://deno.land/std/".to_string())
      .unwrap();
    let output = builder
      .with_loader(|loader| {
        loader.add("/mod.ts", "import 'std/mod.ts'; import '$fresh';");
        loader.add("https://deno.land/std/mod.ts", "export function test() {}");
        loader.add_with_headers(
          "https://localhost/fresh",
          "export function fresh() {}",
          &[("content-type", "application/typescript")],
        );
      })
      .set_original_import_map(original_import_map)
      .build()
      .await
      .unwrap();

    assert_eq!(
      output.import_map,
      Some(json!({
        "imports": {
          "https://deno.land/": "./deno.land/",
          "https://localhost/": "./localhost/",
          "$fresh": "./localhost/fresh.ts",
          "std/mod.ts": "./deno.land/std/mod.ts",
        },
      }))
    );
    assert_eq!(
      output.files,
      to_file_vec(&[
        ("/vendor/deno.land/std/mod.ts", "export function test() {}"),
        ("/vendor/localhost/fresh.ts", "export function fresh() {}")
      ]),
    );
  }

  #[tokio::test]
  async fn existing_import_map_remote_absolute_specifier_local() {
    let mut builder = VendorTestBuilder::with_default_setup();
    let mut original_import_map = builder.new_import_map("/import_map.json");
    original_import_map
      .imports_mut()
      .append(
        "https://localhost/logger.ts?test".to_string(),
        "./local/logger.ts".to_string(),
      )
      .unwrap();

    let output = builder
      .with_loader(|loader| {
        loader.add("/mod.ts", "import 'https://localhost/mod.ts'; import 'https://localhost/logger.ts?test';");
        loader.add("/local/logger.ts", "export class Logger {}");
        // absolute specifier in a remote module that will point at ./local/logger.ts
        loader.add("https://localhost/mod.ts", "import '/logger.ts?test';");
        loader.add("https://localhost/logger.ts?test", "export class Logger {}");
      })
      .set_original_import_map(original_import_map)
      .build()
      .await
      .unwrap();

    assert_eq!(
      output.import_map,
      Some(json!({
        "imports": {
          "https://localhost/logger.ts?test": "../local/logger.ts",
          "https://localhost/": "./localhost/",
        },
        "scopes": {
          "./localhost/": {
            "/logger.ts?test": "../local/logger.ts",
          },
        }
      }))
    );
    assert_eq!(
      output.files,
      to_file_vec(&[("/vendor/localhost/mod.ts", "import '/logger.ts?test';")]),
    );
  }

  #[tokio::test]
  async fn existing_import_map_imports_output_dir() {
    let mut builder = VendorTestBuilder::with_default_setup();
    let mut original_import_map = builder.new_import_map("/import_map.json");
    original_import_map
      .imports_mut()
      .append(
        "std/mod.ts".to_string(),
        "./vendor/deno.land/std/mod.ts".to_string(),
      )
      .unwrap();
    let err = builder
      .with_loader(|loader| {
        loader.add("/mod.ts", "import 'std/mod.ts';");
        loader.add("/vendor/deno.land/std/mod.ts", "export function f() {}");
        loader.add("https://deno.land/std/mod.ts", "export function f() {}");
      })
      .set_original_import_map(original_import_map)
      .build()
      .await
      .err()
      .unwrap();

    assert_eq!(
      err.to_string(),
      concat!(
        "Providing an existing import map with entries for the output ",
        "directory is not supported ",
        "(\"std/mod.ts\": \"./vendor/deno.land/std/mod.ts\").",
      )
    );
  }

  #[tokio::test]
  async fn existing_import_map_scopes_entry_output_dir() {
    let mut builder = VendorTestBuilder::with_default_setup();
    let mut original_import_map = builder.new_import_map("/import_map.json");
    let scopes = original_import_map
      .get_or_append_scope_mut("./other/")
      .unwrap();
    scopes
      .append("/mod.ts".to_string(), "./vendor/mod.ts".to_string())
      .unwrap();
    let err = builder
      .with_loader(|loader| {
        loader.add("/mod.ts", "console.log(5);");
      })
      .set_original_import_map(original_import_map)
      .build()
      .await
      .err()
      .unwrap();

    assert_eq!(
      err.to_string(),
      concat!(
        "Providing an existing import map with entries for the output ",
        "directory is not supported ",
        "(\"/mod.ts\": \"./vendor/mod.ts\").",
      )
    );
  }

  #[tokio::test]
  async fn existing_import_map_scopes_key_output_dir() {
    let mut builder = VendorTestBuilder::with_default_setup();
    let mut original_import_map = builder.new_import_map("/import_map.json");
    let scopes = original_import_map
      .get_or_append_scope_mut("./vendor/")
      .unwrap();
    scopes
      .append("/mod.ts".to_string(), "./vendor/mod.ts".to_string())
      .unwrap();
    let err = builder
      .with_loader(|loader| {
        loader.add("/mod.ts", "console.log(5);");
      })
      .set_original_import_map(original_import_map)
      .build()
      .await
      .err()
      .unwrap();

    assert_eq!(
      err.to_string(),
      concat!(
        "Providing an existing import map with a scope for the output ",
        "directory is not supported (\"./vendor/\").",
      )
    );
  }

  #[tokio::test]
  async fn existing_import_map_http_key() {
    let mut builder = VendorTestBuilder::with_default_setup();
    let mut original_import_map = builder.new_import_map("/import_map.json");
    original_import_map
      .imports_mut()
      .append(
        "http/".to_string(),
        "https://deno.land/std/http/".to_string(),
      )
      .unwrap();
    let output = builder
      .with_loader(|loader| {
        loader.add("/mod.ts", "import 'http/mod.ts';");
        loader.add("https://deno.land/std/http/mod.ts", "console.log(5);");
      })
      .set_original_import_map(original_import_map)
      .build()
      .await
      .unwrap();
    assert_eq!(
      output.import_map,
      Some(json!({
        "imports": {
          "http/mod.ts": "./deno.land/std/http/mod.ts",
          "https://deno.land/": "./deno.land/",
        }
      }))
    );
    assert_eq!(
      output.files,
      to_file_vec(&[("/vendor/deno.land/std/http/mod.ts", "console.log(5);")]),
    );
  }

  #[tokio::test]
  async fn existing_import_map_jsx_import_source_jsx_files() {
    let mut builder = VendorTestBuilder::default();
    builder.add_entry_point("/mod.tsx");
    builder.set_jsx_import_source_config(JsxImportSourceConfig {
      default_specifier: Some("preact".to_string()),
      module: "jsx-runtime".to_string(),
      base_url: builder.resolve_to_url("/deno.json"),
    });
    let mut original_import_map = builder.new_import_map("/import_map.json");
    let imports = original_import_map.imports_mut();
    imports
      .append(
        "preact/".to_string(),
        "https://localhost/preact/".to_string(),
      )
      .unwrap();
    let output = builder
      .with_loader(|loader| {
        loader.add("/mod.tsx", "const myComponent = <div></div>;");
        loader.add_with_headers(
          "https://localhost/preact/jsx-runtime",
          "export function stuff() {}",
          &[("content-type", "application/typescript")],
        );
      })
      .set_original_import_map(original_import_map)
      .build()
      .await
      .unwrap();

    assert_eq!(
      output.import_map,
      Some(json!({
        "imports": {
          "https://localhost/": "./localhost/",
          "preact/jsx-runtime": "./localhost/preact/jsx-runtime.ts",
        },
      }))
    );
    assert_eq!(
      output.files,
      to_file_vec(&[(
        "/vendor/localhost/preact/jsx-runtime.ts",
        "export function stuff() {}"
      ),]),
    );
  }

  #[tokio::test]
  async fn existing_import_map_jsx_import_source_no_jsx_files() {
    let mut builder = VendorTestBuilder::default();
    builder.add_entry_point("/mod.ts");
    builder.set_jsx_import_source_config(JsxImportSourceConfig {
      default_specifier: Some("preact".to_string()),
      module: "jsx-runtime".to_string(),
      base_url: builder.resolve_to_url("/deno.json"),
    });
    let mut original_import_map = builder.new_import_map("/import_map.json");
    let imports = original_import_map.imports_mut();
    imports
      .append(
        "preact/".to_string(),
        "https://localhost/preact/".to_string(),
      )
      .unwrap();
    let output = builder
      .with_loader(|loader| {
        loader.add("/mod.ts", "import 'https://localhost/mod.ts';");
        loader.add("https://localhost/mod.ts", "console.log(1)");
        loader.add_with_headers(
          "https://localhost/preact/jsx-runtime",
          "export function stuff() {}",
          &[("content-type", "application/typescript")],
        );
      })
      .set_original_import_map(original_import_map)
      .build()
      .await
      .unwrap();

    assert_eq!(
      output.import_map,
      Some(json!({
        "imports": {
          "https://localhost/": "./localhost/",
          "preact/jsx-runtime": "./localhost/preact/jsx-runtime.ts"
        },
      }))
    );
    assert_eq!(
      output.files,
      to_file_vec(&[
        ("/vendor/localhost/mod.ts", "console.log(1)"),
        (
          "/vendor/localhost/preact/jsx-runtime.ts",
          "export function stuff() {}"
        ),
      ]),
    );
  }

  #[tokio::test]
  async fn vendor_file_fails_loading_dynamic_import() {
    let mut builder = VendorTestBuilder::with_default_setup();
    let err = builder
      .with_loader(|loader| {
        loader.add("/mod.ts", "import 'https://localhost/mod.ts';");
        loader.add("https://localhost/mod.ts", "await import('./test.ts');");
        loader.add_failure(
          "https://localhost/test.ts",
          "500 Internal Server Error",
        );
      })
      .build()
      .await
      .err()
      .unwrap();

    assert_eq!(
      test_util::strip_ansi_codes(&err.to_string()),
      concat!(
        "500 Internal Server Error\n",
        "    at https://localhost/mod.ts:1:14"
      )
    );
  }

  fn to_file_vec(items: &[(&str, &str)]) -> Vec<(String, String)> {
    items
      .iter()
      .map(|(f, t)| (f.to_string(), t.to_string()))
      .collect()
  }
}
