# File: rust-clippy/clippy_lints/src/semicolon_if_nothing_returned.rs

在rust-clippy的源代码中，`semicolon_if_nothing_returned.rs`文件的作用是实现了一个clippy警告lint，即当在函数中没有返回值时使用了分号（`;`）语句时，会发出警告。

在Rust中，函数可以有一个返回值，也可以没有返回值。当函数没有返回值时，可以省略`->`符号和返回类型。这被称为“隐式返回”。

lint的目的是为了指出可能是错误或不推荐的代码，以帮助开发者编写更好、更安全的代码。检查函数中是否使用了多余的分号语句可以提醒开发者注意可能存在的错误，因为分号语句在隐式返回函数中是不必要的。

该lint的实现位于`semicolon_if_nothing_returned`模块中，其中包含一个名为`check_crate`的函数。该函数遍历了AST（抽象语法树）中的每个函数，对函数体进行检查，找出可能存在多余分号语句的情况，并发出相应的警告。

具体实现细节可参考`semicolon_if_nothing_returned.rs`文件中的代码。

