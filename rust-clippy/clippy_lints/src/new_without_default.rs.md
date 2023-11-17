# File: rust-clippy/clippy_lints/src/new_without_default.rs

在rust-clippy库中，rust-clippy/clippy_lints/src/new_without_default.rs文件实现了一个名为NewWithoutDefault的lint规则，用于检查是否对于实现了Drop trait的类型，使用了没有实现Default trait的值去调用new函数。

NewWithoutDefault是一个struct，其中包含一个fields字段，类型为Vec<Spanned<ast::Ident>>，用于存储源代码中所有使用了没有实现Default trait的值去调用new函数的情况。

NewWithoutDefault结构体的作用是记录在代码中使用了没有实现Default trait的值去调用new函数的位置信息，以便后续的代码检查和警告。

具体的实现细节可以在rust-clippy库的源代码中查看。

