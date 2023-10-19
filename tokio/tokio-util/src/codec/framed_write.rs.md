# File: tokio/tokio-util/src/codec/framed_write.rs

在tokio-util库中的`framed_write.rs`文件中，定义了`FramedWrite`类型和相关的结构体，提供了一种将异步写操作封装在帧（frame）中的方式。

`FramedWrite`是一个实现了`AsyncWrite`和`Sink` trait的类型，它是一个异步写操作的包装器，用于将写入的数据划分为帧。它可以将输入的数据收集起来，在计量到达特定的大小或满足其他条件时将数据封装成帧，并将其写入到底层的异步写入器中，通常是实现了`AsyncWrite`的TcpStream或TcpSink。

`FramedWrite`结构体的定义如下：
```rust
pub struct FramedWrite<T, C> {
    inner: C,
    buffer: Vec<u8>,
    item: PhantomData<T>,
}
```
- `T`是帧的类型；
- `C`是底层异步写入器的类型；
- `inner`字段是底层异步写入器的实例；
- `buffer`字段是用于收集帧数据的缓冲区；
- `item`字段是用于标记泛型参数`T`。

`FramedWrite`提供了几个方法：
- `new`：用于创建一个`FramedWrite`实例；
- `get_mut`：返回底层异步写入器的可变引用；
- `into_inner`：将`FramedWrite`实例还原成底层异步写入器的实例；
- `send`：将待发送的数据封装成帧，并写入到底层异步写入器中；
- `poll_flush`：触发底层异步写入器的`flush`操作，即将缓冲区的数据写入到底层的写入器中；
- `poll_shutdown`：触发底层异步写入器的`shutdown`操作，关闭写操作。

此外，在`framed_write.rs`中还定义了一个内部结构体`FramedWriteSink`，它用于实现`Sink` trait。这个结构体主要负责将输入的数据进行帧封装，并将其写入到`FramedWrite`中。

通过使用`FramedWrite`，我们可以更方便地在异步写入操作中处理帧的概念，尤其适用于需要将数据分块处理并封装成帧的场景，例如TCP通信中的消息传递。

