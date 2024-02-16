# File: /Users/fliter/rust-contribute/rustfmt/src/format_report_formatter.rs

在Rust的rustfmt项目中，/Users/fliter/rust-contribute/rustfmt/src/format_report_formatter.rs是一个源代码文件，它定义了用于格式化报告信息的工具。

在这个文件中，有两个主要的结构体：FormatReportFormatterBuilder<'a>和FormatReportFormatter<'a>。

1. FormatReportFormatterBuilder<'a>结构体被用于构建FormatReportFormatter<'a>的实例。它包含了一系列方法和选项，通过这些方法和选项可以配置和定制生成的报告的格式。

2. FormatReportFormatter<'a>结构体是实际执行格式化并生成报告的工具。通过调用FormatReportFormatterBuilder<'a>的方法，我们可以配置FormatReportFormatter<'a>的选项和行为。然后，我们可以使用FormatReportFormatter<'a>的format方法来生成格式化的报告。该结构体还提供了一些其他方法，用于处理代码片段和生成对应的报告。

总的来说，/Users/fliter/rust-contribute/rustfmt/src/format_report_formatter.rs文件中的FormatReportFormatterBuilder<'a>和FormatReportFormatter<'a>结构体一起提供了一个灵活且可定制的报告格式化工具，用于生成可读性高且易于理解的报告信息。

