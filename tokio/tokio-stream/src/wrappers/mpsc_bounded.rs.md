# File: tokio/tokio-stream/src/wrappers/mpsc_bounded.rs

tokio源代码中的tokio-stream/wrappers/mpsc_bounded.rs文件定义了一个包装器(Wrapper)，名为`ReceiverStream<T>`，它是基于tokio的多生产者单消费者通道（`mpsc`）实现的。

`mpsc`通道是一种在多个生产者与单个消费者之间传递消息的机制。它允许多个生产者向通道发送消息，而这些消息将按照发送的顺序被单个消费者接收。

`ReceiverStream<T>`通过包装`mpsc`通道的接收端来提供一个异步 `Stream`，它可以被等待（awaited）或者转化为其他更高层的抽象，如 `Sink` 或者 `broadcast` 。

以下是`mpsc_bounded.rs`文件中的几个重要结构体的说明：

1. `ReceiverStream<T>`：这是`ReceiverStream<Item = T>`的主要结构体，它实现了异步`Stream` trait，并包装了`mpsc`通道的接收端。通过对该结构体进行调用和操作，可以等待接收端上的消息，获取通道中的数据。

2. `Recv<T>`：这是`ReceiverStream<T>`内部使用的底层接收端结构体。`Recv`结构体持有一个`mpsc`中的接收端`Receiver<T>`。当使用`poll_next`等函数时，它会等待新的消息到达并返回给用户。

3. `InflightBuffer<T>`：这是在`mpsc`通道上缓存未读消息的结构体。当通道产生消息，但接收端还没有处理完所有消息时，这些未处理的消息会被暂存在`InflightBuffer`中。`InflightBuffer`保持追踪下一个预期元素，并在必要时更新缓存。

总结起来，tokio-stream/wrappers/mpsc_bounded.rs文件中定义的`ReceiverStream<T>`结构体提供了一个异步的`Stream`，通过包装tokio的多生产者单消费者（`mpsc`）通道的接收端来实现。这个结构体允许用户等待和获取通道中的消息，并进行进一步的操作。

