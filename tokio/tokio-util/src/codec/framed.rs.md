# File: tokio/tokio-util/src/codec/framed.rs

在tokio-util crate的framed.rs文件中，定义了用于构建和处理编解码器的结构体和函数。

首先是`Framed<T, U>`结构体，它是一个异步编解码器，它使用了`T`类型来表示底层的 I/O 对象（如TcpStream），使用`U`类型表示编解码器。`Framed`结构体提供了对编解码器进行读写操作的方法，以及处理读写错误、关闭连接等操作的功能。

接下来是`FramedParts<T, U>`结构体，它是`Framed`结构体的拆分，用于在取消 `Framed` 结构体之后，获取它的底层 I/O 对象和编解码器。这个结构体可以在编写自定义代码时使用，以获取`Framed`结构体中的底层 I/O 对象和编解码器。

`framed`模块还提供了一些辅助函数和宏，用于创建和配置编解码器。例如，`Builder`结构体用于配置和构建`Framed`对象，`length_delimited`函数用于创建一个以长度为前缀的帧编解码器，`length_field`函数用于创建一个带有长度字段的帧编解码器，`message`函数用于创建一个消息编解码器。

总结一下，tokio-util crate的framed.rs文件提供了构建和处理编解码器的功能，提供了`Framed<T, U>`结构体用于异步编解码操作，`FramedParts<T, U>`结构体用于获取`Framed`的底层 I/O 对象和编解码器。它还提供了一些辅助函数和宏，用于创建和配置编解码器。

