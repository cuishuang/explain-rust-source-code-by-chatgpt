# File: /Users/fliter/rust-contribute/rustfmt/src/config/options.rs

在rustfmt项目的源代码中，/Users/fliter/rust-contribute/rustfmt/src/config/options.rs文件是用于定义rustfmt的配置选项的。该文件中包含了一些结构体、trait和枚举。

WidthHeuristics结构体用于存储表达式宽度的启发式规则。它包含了列宽的最大限制、tab宽度、连续行前缩进宽度等参数。

IgnoreList结构体用于存储rustfmt忽略的文件列表。它包含了一个向量，用于存储需要被忽略的文件或文件夹路径。

HashSetVisitor结构体是用于访问HashSet的结构体。它实现了HashSet的访问接口。

CliOptions trait是一个用于命令行选项的trait。它定义了解析命令行选项、输出帮助信息、生成默认配置等操作的接口。

NewlineStyle enum定义了换行风格的选项，包括UNIX、Windows和系统默认风格。

BraceStyle enum定义了花括号风格的选项，包括SameLine、NextLine等。

ControlBraceStyle enum定义了控制结构的花括号风格选项，包括AlwaysSameLine、AlwaysNextLine等。

IndentStyle enum定义了缩进风格的选项，包括Block、Visual等。

Density enum定义了代码块的密度选项，包括Tall、Compact等。

TypeDensity enum定义了类型的密度选项，包括Wide、Tall等。

Heuristics enum定义了代码布局的启发式规则选项，包括Enable、Disable等。

GroupImportsTactic enum定义了导入声明分组的策略选项，包括Mixed、Leads等。

ImportGranularity enum定义了导入粒度的选项，包括Crate、Module等。

HexLiteralCase enum定义了十六进制字面量的大小写选项，包括Lower、Upper等。

ReportTactic enum定义了错误报告策略的选项，包括Fail、Warn等。

EmitMode enum定义了输出模式的选项，包括Files、Modified等。

Color enum定义了颜色输出的选项，包括Auto、Always等。

Version enum定义了rustfmt版本的选项，包括No、Yes等。

Verbosity enum定义了详细程度的选项，包括Quiet、Verbose等。

Edition enum定义了Rust编程语言的版本的选项，包括2015、2018等。

MatchArmLeadingPipe enum定义了匹配分支的竖线的位置的选项，包括Match、PreferNextLine等。

