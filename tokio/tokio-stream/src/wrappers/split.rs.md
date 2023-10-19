# File: tokio/tokio-stream/src/wrappers/split.rs

在tokio源代码中，tokio-stream/src/wrappers/split.rs文件的作用是为实现流（Stream）的切分提供支持。这个文件中定义了SplitStream<R>结构体，用来将一个包含多个元素的流切分为单个的元素流，并提供一些操作方法。

SplitStream<R>是一个泛型结构体，其中的R类型参数表示要切分的原始流。它实现了futures::stream::Stream trait，因此可以使用Stream trait提供的方法来操作切分后的流。

SplitStream<R>结构体的主要作用是将原始流切分为单个元素流。切分后的元素流是由tokio::sync::mpsc::Receiver<T>类型表示的，其中T是原始流中的元素类型。SplitStream<R>结构体内部会创建一个mpsc通道（即多个生产者、单个消费者通道），用于从原始流中获取元素并发送到切分后的元素流。

SplitStream<R>结构体提供了一些方法来处理切分后的元素流。这些方法包括：

1. poll_next: 实现futures::stream::Stream trait的方法，用于从切分后的元素流中获取下一个元素。当切分后的元素流中没有元素时，该方法会返回Poll::Pending，否则返回Poll::Ready(Some(item))。

2. try_next: 类似于poll_next方法，但是它不是轮询式地获取下一个元素，而是立即返回（非阻塞）。当切分后的元素流中有元素时，该方法会返回Some(item)，否则返回None。

3. into_inner: 将SplitStream<R>结构体转换为tokio::sync::mpsc::Receiver<T>类型，即获取切分后的元素流的Receiver。这可以用于进一步操作原始流中的元素。

总结来说，SplitStream<R>结构体的作用是将原始流切分为单个元素流，提供了一些方法来操作切分后的元素流，以及获取原始流中的元素。

