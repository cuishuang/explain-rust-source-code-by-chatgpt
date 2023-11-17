# File: rust-clippy/clippy_lints/src/excessive_nesting.rs

在rust-clippy库的源代码中，rust-clippy/clippy_lints/src/excessive_nesting.rs文件的作用是实现了对嵌套层数过多的代码进行 lint 检测。

在这个文件中，主要定义了两个 struct：ExcessiveNesting 和 NestingVisitor。

ExcessiveNesting struct 是一个 lint 结构体，其实现了 LintPass trait。它用于指定 lint 的名称、描述和级别，以及定义具体的 lint 规则。

NestingVisitor struct 是巡查器，也就是实际去访问代码并进行 lint 检测的结构体。它实现了 rustc_ast::visit::Visitor trait，通过重写其中的方法来遍历代码，并对嵌套层数进行检查。NestingVisitor 的主要作用是在代码中搜索深度超过指定阈值的嵌套结构，并向 ExcessiveNesting 结构体报告这些问题。

在 NestingVisitor 中，通过重写 Visitor trait 中的方法，可以在访问AST（抽象语法树）的不同节点时进行特定的处理。比如，在访问一个语句块（block）节点时，可以获取其内部的嵌套层数，如果超过了阈值，则可以使用 ExcessiveNesting 的报告方法进行错误报告。通过遍历整个代码，并递归地检查嵌套层数，可以找到所有可能的问题。

总结：excessive_nesting.rs 文件中的 ExcessiveNesting 和 NestingVisitor 结构体分别定义了 lint 检测的规则和实际的访问者，用于检测代码中的嵌套层数过多的问题，并生成相应的 lint 报告。

