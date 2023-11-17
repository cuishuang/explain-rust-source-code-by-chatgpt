# File: rust-clippy/clippy_lints/src/swap.rs

在rust-clippy的源代码中，`rust-clippy/clippy_lints/src/swap.rs`文件是一个用于定义和实现swap相关的lint的模块。

该文件中定义了一个名为`check_swap`的函数，该函数用于检查代码中的swap操作，并根据特定规则给出相关的警告信息。

`ExprOrIdent<'a>`是一个枚举类型，用于表示表达式或标识符。它定义了两个变体：

1. `Expr`变体：用于包装表达式，表示一个由语法树节点表示的表达式。
2. `Ident`变体：用于包装标识符，表示一个由标识符字符串表示的标识符。

这个枚举的作用是在具体的实现中，用于处理不同类型的输入参数并进行相应的操作，如检查swap操作时会根据具体的表达式或标识符进行相关的检查和警告。

