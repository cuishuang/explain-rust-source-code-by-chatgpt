# File: tokio/tokio-stream/src/stream_ext/fuse.rs

在tokio源代码中，tokio-stream/src/stream_ext/fuse.rs文件的作用是实现了一个可以将流（stream）的特性转换为双端迭代器（bidirectional iterator）的适配器。这个适配器使得流可以在发送和接收元素之间切换。

具体来说，这个文件中定义了一个名为Fuse的结构体，它是一个用于流的适配器。Fuse结构体实现了Stream和FusedStream这两个trait，它的作用是在迭代过程中将流的状态逻辑与元素的迭代逻辑分开，并提供了一系列方法来修改和获取这个状态。这个结构体有三个字段：

1. stream: T，它代表了要适配的流对象。这个流对象的类型必须实现了Stream trait。
2. is_done: bool，用于标识流是否已经结束。当流结束后，is_done会被设置为true。
3. is_complete: bool，表示流是否已经完全消耗。初始时，is_complete为false，表示还有元素可以消耗，当流被完全消耗后，is_complete会被设置为true。

Fuse结构体提供了一系列方法，用于操作和获取流的状态，包括：

1. new(stream: T) -> Fuse<T>：根据给定的流对象创建一个Fuse适配器。
2. set_done(&mut self)：将is_done字段设置为true，表示流已经结束。
3. is_done(&self) -> bool：判断流是否已经结束。
4. set_complete(&mut self)：将is_complete字段设置为true，表示流已经完全消耗。
5. is_complete(&self) -> bool：判断流是否已经完全消耗。
6. into_inner(self) -> T：将Fuse结构体恢复为内部的流对象。

通过使用Fuse适配器，可以将流对象作为双端迭代器来使用，能够更加方便地在发送和接收元素之间切换，提高代码的复用性和灵活性。

