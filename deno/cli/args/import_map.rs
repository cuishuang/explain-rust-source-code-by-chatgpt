// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use deno_core::error::AnyError;
use deno_core::serde_json;
use deno_core::url::Url;
use deno_runtime::permissions::PermissionsContainer;
use import_map::ImportMap;
use import_map::ImportMapDiagnostic;
use log::warn;

use super::ConfigFile;
use crate::file_fetcher::FileFetcher;

pub async fn resolve_import_map_from_specifier(
  specifier: &Url,
  maybe_config_file: Option<&ConfigFile>,
  file_fetcher: &FileFetcher,
) -> Result<ImportMap, AnyError> {
  let value: serde_json::Value = if specifier.scheme() == "data" {
    let data_url_text =
      deno_graph::source::RawDataUrl::parse(specifier)?.decode()?;
    serde_json::from_str(&data_url_text)?
  } else {
    let import_map_config = maybe_config_file
      .as_ref()
      .filter(|c| c.specifier == *specifier);
    match import_map_config {
      Some(config) => config.to_import_map_value(),
      None => {
        let file = file_fetcher
          .fetch(specifier, PermissionsContainer::allow_all())
          .await?
          .into_text_decoded()?;
        serde_json::from_str(&file.source)?
      }
    }
  };
  import_map_from_value(specifier, value)
}

pub fn import_map_from_value(
  specifier: &Url,
  json_value: serde_json::Value,
) -> Result<ImportMap, AnyError> {
  debug_assert!(
    !specifier.as_str().contains("../"),
    "Import map specifier incorrectly contained ../: {}",
    specifier.as_str()
  );
  let result = import_map::parse_from_value(specifier, json_value)?;
  print_import_map_diagnostics(&result.diagnostics);
  Ok(result.import_map)
}

fn print_import_map_diagnostics(diagnostics: &[ImportMapDiagnostic]) {
  if !diagnostics.is_empty() {
    warn!(
      "Import map diagnostics:\n{}",
      diagnostics
        .iter()
        .map(|d| format!("  - {d}"))
        .collect::<Vec<_>>()
        .join("\n")
    );
  }
}
