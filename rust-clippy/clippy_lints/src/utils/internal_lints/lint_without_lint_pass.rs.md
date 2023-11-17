# File: rust-clippy/clippy_lints/src/utils/internal_lints/lint_without_lint_pass.rs

在rust-clippy项目中，`lint_without_lint_pass.rs`文件定义了`LintWithoutLintPass`和`LintCollector`两个结构体，主要用于收集没有通过`lint pass`的lint。

`LintWithoutLintPass`结构体是定义了一个未通过`lint pass`的lint的相关信息，包括文件路径、行数、所在函数等。它的成员字段有：
- `span`：表示在源代码中的位置范围。
- `id`：表示lint的标识符，在rust-clippy中通常是通过`LintId`表示的。
- `desc`：表示lint的描述信息；
- `lint_level`：表示lint的严重程度，通常是通过`rustc::lint::Level`表示的；
- `details`：表示lint的详细信息。

`LintCollector`结构体是用于收集没有通过`lint pass`的lint的工具，它的成员字段有：
- `ctx`：表示rust-clippy工作的上下文环境，通过`&'a LateContext<'a>`指定。
- `info`：表示收集到的lint信息，是一个可变的`Vec<LintWithoutLintPass>`。

`LintCollector`结构体实现了`LateLintPass` trait，用于在rust-clippy的lint过程中进行调用和调度。它主要的方法有：
- `check_expr`：在处理表达式过程中进行lint检查；
- `check_stmt`：在处理语句过程中进行lint检查；
- `collect`：用于收集未通过`lint pass`的lint信息，将其添加到`LintCollector`的`info`字段中。

通过`LintCollector`结构体以及相关方法，可以实现对未通过`lint pass`的lint的收集，便于后续的分析和处理。

