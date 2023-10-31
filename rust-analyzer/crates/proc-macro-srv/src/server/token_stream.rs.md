# File: rust-analyzer/crates/proc-macro-srv/src/server/token_stream.rs

在rust-analyzer的源代码中，`token_stream.rs`文件是`proc-macro-srv` crate（处理过程宏服务）的一部分。它包含用于处理token流的结构体和函数的定义。

`TokenStream`结构体是一个不可变的Token流，表示一个由词法单元组成的序列。该结构体的主要作用是提供操作Token流的方法，例如迭代、合并、拆分等。

`TokenStreamBuilder`结构体是用于构建Token流的工具。它提供了一组方法来添加Token到构建器，并最终构建为一个完整的`TokenStream`对象。`TokenStreamBuilder`可以用来生成Token流，并帮助构建和修改过程宏的输出。

这两个结构体的作用是支持过程宏的生成和处理。过程宏是Rust编程语言中的一项强大的元编程功能，允许开发者编写自定义的代码转换器。通过操作和分析Token流，可以实现各种复杂的代码转换和代码生成。

在`token_stream.rs`文件中，还定义了一些其他的函数和结构体，用于处理Token流的具体操作，例如合并Token、拆分Token、检查Token类型等。这些函数和结构体提供了基本的Token流操作接口，为过程宏服务提供了底层的支持和功能。

总结起来，`token_stream.rs`文件中的`TokenStream`结构体和`TokenStreamBuilder`结构体提供了处理Token流的工具和方法，用于支持过程宏的生成和处理。通过这些结构体和相关函数，可以实现复杂的代码转换和代码生成。

