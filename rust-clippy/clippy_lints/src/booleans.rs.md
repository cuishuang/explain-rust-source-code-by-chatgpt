# File: rust-clippy/clippy_lints/src/booleans.rs

在rust-clippy的源代码中，`rust-clippy/clippy_lints/src/booleans.rs`文件的作用是定义了一些与布尔值（Booleans）相关的lint检查。

具体来说，`NonminimalBoolVisitor`结构体是一个访问者（visitor），它用于遍历Rust代码的抽象语法树（AST）并检查其中的布尔值相关问题。它实现了`clippy_lints::utils::Visitor` trait，并重载了该trait中的方法来执行具体的检查操作。

`Hir2Qmm`结构体是对Hyper 模块 HIR（高级中间表示）的进行了变换，使其更适用于后续的检查阶段。

`SuggestContext`结构体是用于提供检查相关的上下文信息，如源码位置等。它使用了泛型参数来指定具体的访问者类型。

`Stats`结构体用于记录进行了多少次具体的检查操作。

`NotSimplificationVisitor`结构体是一个访问者，用于遍历和检查不简化的布尔表达式。

这些结构体协同工作，对布尔值相关的代码进行lint检查，并提供了上下文信息以及统计数据等功能。

