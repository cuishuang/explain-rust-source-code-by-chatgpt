# File: rust-clippy/clippy_lints/src/await_holding_invalid.rs

在rust-clippy项目中，rust-clippy/clippy_lints/src/await_holding_invalid.rs文件的作用是实现Rust语法规则检查器的一个lint规则，用于检查`await`表达式后面是否跟着Future对象的变量。

具体而言，该lint规则旨在发现并警告使用了不合适的`await`表达式，即在`await`之后直接跟着非Future类型的变量。这种情况通常是由于程序员错误地将非Future值视为Future值，并试图在其上使用`await`导致的。

在该文件中，定义了一个名为`AwaitHoldingInvalid`的struct，用于实现相关的lint规则。该struct主要包含两个方法：`lint_async`和`lint_async_mut`。这两个方法分别处理异步函数和异步可变函数的情况。

`lint_async`方法接收函数的参数列表、函数体和函数的Span，通过调用`check_expr`方法对函数体中的表达式进行检查，并根据检查结果调用`span_lint_and_sugg`方法生成警告信息。

`lint_async_mut`方法与`lint_async`方法类似，但专门处理异步可变函数的情况。

总的来说，`AwaitHoldingInvalid`struct的作用是实现lint规则，对于使用了不合适的`await`表达式的情况，提供警告信息给开发者以便发现并修复潜在的错误。

