# File: rust-analyzer/crates/hir-expand/src/proc_macro.rs

在rust-analyzer的源代码中，`proc_macro.rs`文件位于`rust-analyzer/crates/hir-expand/src/`目录下，它的作用是将`proc_macro`类型的节点扩展为宏展开的语法树节点。

在Rust中，`proc_macro`是一个特殊的属性宏，可以用来对 Rust 代码进行编译时代码生成。`proc_macro.rs`模块实现了处理并展开这些属性宏的功能，使得 rust-analyzer 能够正确解析和分析这些宏的语义。

在`proc_macro.rs`文件中，`ProcMacroExpander`是一个结构体，代表了一个 proc_macro 展开器，它有三个字段：

1. `expansion`：表示宏展开结果的语法树节点
2. `remap`：一个哈希表，用于记录宏展开后，原来节点的位置和新节点的位置之间的对应关系
3. `abort`：一个标志，用于表示是否遇到了展开宏时的错误，如果为 true，则展开宏的操作会被中止

这个结构体的作用是扩展来自语法树的`proc_macro`节点。它会遍历 AST 中的每个节点，遇到`proc_macro`节点就将其替换为宏展开的结果。它还会同时更新`remap`哈希表来维护节点的对应关系。

此外，还有另外两个结构体与`ProcMacroExpander`有关：

1. `ProcMacroAttributeExpander`：继承自`AstVisitor`，用于处理属性宏的展开，完成展开的结果保存在`ProcMacroExpander`的`expansion`字段中。

2. `ProcMacroDeriveExpander`：继承自`AstVisitor`，用于处理派生宏的展开，也会将展开结果保存到`ProcMacroExpander`的`expansion`字段中。

总结起来，`proc_macro.rs`文件实现了实际处理 proc_macro 展开的逻辑，`ProcMacroExpander`结构体则代表着一个 proc_macro 展开器，负责将 proc_macro 展开为语法树节点，并维护展开节点与原节点的对应关系。

