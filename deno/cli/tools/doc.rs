// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use crate::args::DocFlags;
use crate::args::DocHtmlFlag;
use crate::args::DocSourceFileFlag;
use crate::args::Flags;
use crate::colors;
use crate::diagnostics::Diagnostic;
use crate::diagnostics::DiagnosticLevel;
use crate::diagnostics::DiagnosticLocation;
use crate::diagnostics::DiagnosticSnippet;
use crate::diagnostics::DiagnosticSnippetHighlight;
use crate::diagnostics::DiagnosticSnippetHighlightStyle;
use crate::diagnostics::DiagnosticSnippetSource;
use crate::diagnostics::DiagnosticSourcePos;
use crate::diagnostics::DiagnosticSourceRange;
use crate::diagnostics::SourceTextParsedSourceStore;
use crate::display::write_json_to_stdout;
use crate::display::write_to_stdout_ignore_sigpipe;
use crate::factory::CliFactory;
use crate::graph_util::graph_lock_or_exit;
use crate::tsc::get_types_declaration_file_text;
use crate::util::fs::collect_specifiers;
use deno_config::glob::FilePatterns;
use deno_config::glob::PathOrPatternSet;
use deno_core::anyhow::bail;
use deno_core::anyhow::Context;
use deno_core::error::AnyError;
use deno_core::futures::FutureExt;
use deno_doc as doc;
use deno_graph::GraphKind;
use deno_graph::ModuleAnalyzer;
use deno_graph::ModuleParser;
use deno_graph::ModuleSpecifier;
use doc::DocDiagnostic;
use doc::DocDiagnosticKind;
use indexmap::IndexMap;
use lsp_types::Url;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::rc::Rc;

async fn generate_doc_nodes_for_builtin_types(
  doc_flags: DocFlags,
  parser: &dyn ModuleParser,
  analyzer: &dyn ModuleAnalyzer,
) -> Result<IndexMap<ModuleSpecifier, Vec<doc::DocNode>>, AnyError> {
  let source_file_specifier =
    ModuleSpecifier::parse("internal://lib.deno.d.ts").unwrap();
  let content = get_types_declaration_file_text();
  let mut loader = deno_graph::source::MemoryLoader::new(
    vec![(
      source_file_specifier.to_string(),
      deno_graph::source::Source::Module {
        specifier: source_file_specifier.to_string(),
        content,
        maybe_headers: None,
      },
    )],
    Vec::new(),
  );
  let mut graph = deno_graph::ModuleGraph::new(GraphKind::TypesOnly);
  graph
    .build(
      vec![source_file_specifier.clone()],
      &mut loader,
      deno_graph::BuildOptions {
        module_analyzer: Some(analyzer),
        ..Default::default()
      },
    )
    .await;
  let doc_parser = doc::DocParser::new(
    &graph,
    parser,
    doc::DocParserOptions {
      diagnostics: false,
      private: doc_flags.private,
    },
  )?;
  let nodes = doc_parser.parse_module(&source_file_specifier)?.definitions;

  Ok(IndexMap::from([(source_file_specifier, nodes)]))
}

pub async fn doc(flags: Flags, doc_flags: DocFlags) -> Result<(), AnyError> {
  let factory = CliFactory::from_flags(flags).await?;
  let cli_options = factory.cli_options();
  let module_info_cache = factory.module_info_cache()?;
  let parsed_source_cache = factory.parsed_source_cache();
  let capturing_parser = parsed_source_cache.as_capturing_parser();
  let analyzer = module_info_cache.as_module_analyzer(&capturing_parser);

  let doc_nodes_by_url = match doc_flags.source_files {
    DocSourceFileFlag::Builtin => {
      generate_doc_nodes_for_builtin_types(
        doc_flags.clone(),
        &capturing_parser,
        &analyzer,
      )
      .await?
    }
    DocSourceFileFlag::Paths(ref source_files) => {
      let module_graph_builder = factory.module_graph_builder().await?;
      let maybe_lockfile = factory.maybe_lockfile();

      let module_specifiers = collect_specifiers(
        FilePatterns {
          base: cli_options.initial_cwd().to_path_buf(),
          include: Some(PathOrPatternSet::from_relative_path_or_patterns(
            cli_options.initial_cwd(),
            source_files,
          )?),
          exclude: Default::default(),
        },
        |_, _| true,
      )?;
      let graph = module_graph_builder
        .create_graph(GraphKind::TypesOnly, module_specifiers.clone())
        .await?;

      if let Some(lockfile) = maybe_lockfile {
        graph_lock_or_exit(&graph, &mut lockfile.lock());
      }

      let doc_parser = doc::DocParser::new(
        &graph,
        &capturing_parser,
        doc::DocParserOptions {
          private: doc_flags.private,
          diagnostics: doc_flags.lint,
        },
      )?;

      let mut doc_nodes_by_url =
        IndexMap::with_capacity(module_specifiers.len());

      for module_specifier in module_specifiers {
        let nodes = doc_parser.parse_with_reexports(&module_specifier)?;
        doc_nodes_by_url.insert(module_specifier, nodes);
      }

      if doc_flags.lint {
        let diagnostics = doc_parser.take_diagnostics();
        check_diagnostics(&**parsed_source_cache, &diagnostics)?;
      }

      doc_nodes_by_url
    }
  };

  if let Some(html_options) = &doc_flags.html {
    let deno_ns = if doc_flags.source_files != DocSourceFileFlag::Builtin {
      let deno_ns = generate_doc_nodes_for_builtin_types(
        doc_flags.clone(),
        &capturing_parser,
        &analyzer,
      )
      .await?;
      let (_, deno_ns) = deno_ns.first().unwrap();

      deno_doc::html::compute_namespaced_symbols(deno_ns, &[])
    } else {
      Default::default()
    };

    generate_docs_directory(&doc_nodes_by_url, html_options, deno_ns)
      .boxed_local()
      .await
  } else {
    let modules_len = doc_nodes_by_url.len();
    let doc_nodes =
      doc_nodes_by_url.into_values().flatten().collect::<Vec<_>>();

    if doc_flags.json {
      write_json_to_stdout(&doc_nodes)
    } else if doc_flags.lint {
      // don't output docs if running with only the --lint flag
      log::info!(
        "Checked {} file{}",
        modules_len,
        if modules_len == 1 { "" } else { "s" }
      );
      Ok(())
    } else {
      print_docs_to_stdout(doc_flags, doc_nodes)
    }
  }
}

struct DocResolver {
  deno_ns: std::collections::HashSet<Vec<String>>,
}

impl deno_doc::html::HrefResolver for DocResolver {
  fn resolve_global_symbol(&self, symbol: &[String]) -> Option<String> {
    if self.deno_ns.contains(symbol) {
      Some(format!(
        "https://deno.land/api@{}?s={}",
        env!("CARGO_PKG_VERSION"),
        symbol.join(".")
      ))
    } else {
      None
    }
  }

  fn resolve_import_href(
    &self,
    symbol: &[String],
    src: &str,
  ) -> Option<String> {
    let mut url = ModuleSpecifier::parse(src).ok()?;

    if url.domain() == Some("deno.land") {
      url.set_query(Some(&format!("s={}", symbol.join("."))));
      return Some(url.to_string());
    }

    None
  }

  fn resolve_usage(
    &self,
    _current_specifier: &ModuleSpecifier,
    current_file: &str,
  ) -> Option<String> {
    Some(current_file.to_string())
  }

  fn resolve_source(&self, location: &deno_doc::Location) -> Option<String> {
    Some(location.filename.clone())
  }
}

async fn generate_docs_directory(
  doc_nodes_by_url: &IndexMap<ModuleSpecifier, Vec<doc::DocNode>>,
  html_options: &DocHtmlFlag,
  deno_ns: std::collections::HashSet<Vec<String>>,
) -> Result<(), AnyError> {
  let cwd = std::env::current_dir().context("Failed to get CWD")?;
  let output_dir_resolved = cwd.join(&html_options.output);

  let options = deno_doc::html::GenerateOptions {
    package_name: Some(html_options.name.to_owned()),
    main_entrypoint: None,
    rewrite_map: None,
    hide_module_doc_title: false,
    href_resolver: Rc::new(DocResolver { deno_ns }),
    sidebar_flatten_namespaces: false,
  };

  let files = deno_doc::html::generate(options, doc_nodes_by_url)
    .context("Failed to generate HTML documentation")?;

  let path = &output_dir_resolved;
  let _ = std::fs::remove_dir_all(path);
  std::fs::create_dir(path)
    .with_context(|| format!("Failed to create directory {:?}", path))?;

  let no_of_files = files.len();
  for (name, content) in files {
    let this_path = path.join(name);
    let prefix = this_path.parent().with_context(|| {
      format!("Failed to get parent path for {:?}", this_path)
    })?;
    std::fs::create_dir_all(prefix)
      .with_context(|| format!("Failed to create directory {:?}", prefix))?;
    std::fs::write(&this_path, content)
      .with_context(|| format!("Failed to write file {:?}", this_path))?;
  }

  log::info!(
    "{}",
    colors::green(format!(
      "Written {} files to {:?}",
      no_of_files, html_options.output
    ))
  );
  Ok(())
}

fn print_docs_to_stdout(
  doc_flags: DocFlags,
  mut doc_nodes: Vec<deno_doc::DocNode>,
) -> Result<(), AnyError> {
  doc_nodes.retain(|doc_node| doc_node.kind != doc::DocNodeKind::Import);
  let details = if let Some(filter) = doc_flags.filter {
    let nodes = doc::find_nodes_by_name_recursively(doc_nodes, filter.clone());
    if nodes.is_empty() {
      bail!("Node {} was not found!", filter);
    }
    format!(
      "{}",
      doc::DocPrinter::new(&nodes, colors::use_color(), doc_flags.private)
    )
  } else {
    format!(
      "{}",
      doc::DocPrinter::new(&doc_nodes, colors::use_color(), doc_flags.private)
    )
  };

  write_to_stdout_ignore_sigpipe(details.as_bytes()).map_err(AnyError::from)
}

impl Diagnostic for DocDiagnostic {
  fn level(&self) -> DiagnosticLevel {
    DiagnosticLevel::Error
  }

  fn code(&self) -> impl std::fmt::Display + '_ {
    match self.kind {
      DocDiagnosticKind::MissingJsDoc => "missing-jsdoc",
      DocDiagnosticKind::MissingExplicitType => "missing-explicit-type",
      DocDiagnosticKind::MissingReturnType => "missing-return-type",
      DocDiagnosticKind::PrivateTypeRef { .. } => "private-type-ref",
    }
  }

  fn message(&self) -> impl std::fmt::Display + '_ {
    match &self.kind {
      DocDiagnosticKind::MissingJsDoc => {
        Cow::Borrowed("exported symbol is missing JSDoc documentation")
      }
      DocDiagnosticKind::MissingExplicitType => {
        Cow::Borrowed("exported symbol is missing an explicit type annotation")
      }
      DocDiagnosticKind::MissingReturnType => Cow::Borrowed(
        "exported function is missing an explicit return type annotation",
      ),
      DocDiagnosticKind::PrivateTypeRef {
        reference, name, ..
      } => Cow::Owned(format!(
        "public type '{name}' references private type '{reference}'",
      )),
    }
  }

  fn location(&self) -> DiagnosticLocation {
    let specifier = Url::parse(&self.location.filename).unwrap();
    DiagnosticLocation::ModulePosition {
      specifier: Cow::Owned(specifier),
      source_pos: DiagnosticSourcePos::ByteIndex(self.location.byte_index),
    }
  }

  fn snippet(&self) -> Option<DiagnosticSnippet<'_>> {
    let specifier = Url::parse(&self.location.filename).unwrap();
    Some(DiagnosticSnippet {
      source: DiagnosticSnippetSource::Specifier(Cow::Owned(specifier)),
      highlight: DiagnosticSnippetHighlight {
        style: DiagnosticSnippetHighlightStyle::Error,
        range: DiagnosticSourceRange {
          start: DiagnosticSourcePos::ByteIndex(self.location.byte_index),
          end: DiagnosticSourcePos::ByteIndex(self.location.byte_index + 1),
        },
        description: None,
      },
    })
  }

  fn hint(&self) -> Option<impl std::fmt::Display + '_> {
    match &self.kind {
      DocDiagnosticKind::PrivateTypeRef { .. } => {
        Some("make the referenced type public or remove the reference")
      }
      _ => None,
    }
  }
  fn snippet_fixed(&self) -> Option<DiagnosticSnippet<'_>> {
    match &self.kind {
      DocDiagnosticKind::PrivateTypeRef {
        reference_location, ..
      } => {
        let specifier = Url::parse(&reference_location.filename).unwrap();
        Some(DiagnosticSnippet {
          source: DiagnosticSnippetSource::Specifier(Cow::Owned(specifier)),
          highlight: DiagnosticSnippetHighlight {
            style: DiagnosticSnippetHighlightStyle::Hint,
            range: DiagnosticSourceRange {
              start: DiagnosticSourcePos::ByteIndex(
                reference_location.byte_index,
              ),
              end: DiagnosticSourcePos::ByteIndex(
                reference_location.byte_index + 1,
              ),
            },
            description: Some(Cow::Borrowed("this is the referenced type")),
          },
        })
      }
      _ => None,
    }
  }

  fn info(&self) -> std::borrow::Cow<'_, [std::borrow::Cow<'_, str>]> {
    match &self.kind {
      DocDiagnosticKind::MissingJsDoc => Cow::Borrowed(&[]),
      DocDiagnosticKind::MissingExplicitType => Cow::Borrowed(&[]),
      DocDiagnosticKind::MissingReturnType => Cow::Borrowed(&[]),
      DocDiagnosticKind::PrivateTypeRef { .. } => {
        Cow::Borrowed(&[Cow::Borrowed(
          "to ensure documentation is complete all types that are exposed in the public API must be public",
        )])
      }
    }
  }

  fn docs_url(&self) -> Option<impl std::fmt::Display + '_> {
    None::<&str>
  }
}

fn check_diagnostics(
  parsed_source_cache: &dyn deno_graph::ParsedSourceStore,
  diagnostics: &[DocDiagnostic],
) -> Result<(), AnyError> {
  if diagnostics.is_empty() {
    return Ok(());
  }

  // group by location then by line (sorted) then column (sorted)
  let mut diagnostic_groups = IndexMap::new();
  for diagnostic in diagnostics {
    diagnostic_groups
      .entry(diagnostic.location.filename.clone())
      .or_insert_with(BTreeMap::new)
      .entry(diagnostic.location.line)
      .or_insert_with(BTreeMap::new)
      .entry(diagnostic.location.col)
      .or_insert_with(Vec::new)
      .push(diagnostic);
  }

  for (_, diagnostics_by_lc) in diagnostic_groups {
    for (_, diagnostics_by_col) in diagnostics_by_lc {
      for (_, diagnostics) in diagnostics_by_col {
        for diagnostic in diagnostics {
          let sources = SourceTextParsedSourceStore(parsed_source_cache);
          eprintln!("{}", diagnostic.display(&sources));
        }
      }
    }
  }
  bail!(
    "Found {} documentation lint error{}.",
    colors::bold(diagnostics.len().to_string()),
    if diagnostics.len() == 1 { "" } else { "s" }
  );
}