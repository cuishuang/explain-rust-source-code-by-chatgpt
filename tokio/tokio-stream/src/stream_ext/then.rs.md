# File: tokio/tokio-stream/src/stream_ext/then.rs

在tokio源代码中，tokio-stream库的stream_ext/then.rs文件定义了扩展方法来处理Future和Stream的组合操作。该文件提供了一个名为`Then`的结构体以及相关的方法和辅助函数。

`Then`结构体是一个实现了`Future` trait的Stream，它可以用于将流(Stream)的每个元素映射为一个Future，并在每个Future完成后处理结果。它的作用是将每个元素进行异步处理，并返回一个新的流。

具体来说，`Then`结构体包含以下字段：

- `stream`: 要处理的流(Stream)对象。
- `fut`: 一个可调用的闭包，它将每个流元素映射为一个Future。

`Then`结构体还实现了一系列方法，如`poll`、`poll_next`、`poll_ready`等，用于处理异步操作。

`Then`结构体的相关方法和辅助函数包括：

- `new`: 用于创建一个新的`Then`实例。
- `poll`: 用于在异步上下文中检查`Then`的状态并处理相关操作。
- `poll_next`: 用于获取`Then`流的下一个元素，在需要时触发处理闭包(fut)。
- `poll_ready`: 用于检查并等待`Then`的闭包(fut)是否已经准备好进行处理。
- `get_mut`: 用于获取对内部流对象的可变引用。
- `into_inner`: 用于获取内部的流对象。

通过使用`Then`结构体和相关方法，开发者可以很方便地将流(Stream)和对每个元素的异步处理操作组合起来，实现复杂的异步流处理逻辑。

