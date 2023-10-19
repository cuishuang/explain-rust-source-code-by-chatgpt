# File: tokio/tokio-stream/src/stream_ext/timeout.rs

stream_ext/timeout.rs 文件是 tokio 中的一个扩展 trait `StreamExt` 的实现文件。该文件定义了 `StreamExt` trait 添加的一个函数 `timeout`，用于为流添加一个超时机制。

`Timeout<S>` 是一个通用的包装类型，它支持超时功能。它实现了 `Stream` trait，并包装了一个要超时的源流 `S`。`Timeout<S>` 内部维护了一个计时器，当源流超过给定的超时时间后，将会产生 `Elapsed` 事件。

`Elapsed(())` 是一个代表超时的事件。它是一个空的元组结构体，因为在该库中超时只是一种代表事件而不是数据。当 `Timeout<S>` 触发超时事件时，将会生成 `Elapsed` 事件。

`Timeout<S>` 是通过 `StreamExt` trait 的扩展函数 `timeout` 创建的。 `timeout` 函数接受一个 `Duration` 参数，表示超时时间。它通过创建 `Timeout` 实例并将其包装在一个新的 `Stream` 中来为原始流添加超时功能。当原始流产生事件时，它会监视超时计时器，并在超时时间到达时生成 `Elapsed` 事件。否则，它将通过将事件传播到原始流来生成数据事件。

使用示例：

```rust
use tokio::time::Duration;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let stream = tokio_stream::iter(vec![1, 2, 3])
        .timeout(Duration::from_secs(2));

    pin_utils::pin_mut!(stream);

    while let Some(item) = stream.next().await {
        match item {
            Ok(value) => println!("Received value: {}", value),
            Err(_) => println!("Timeout occurred!"),
        }
    }
}
```

在上面的示例中，我们使用 `tokio_stream::iter` 创建了一个简单的流，表示依次产生数字1、2、3。然后我们使用 `.timeout(Duration::from_secs(2))` 扩展函数为流添加了一个超时时间为 2 秒的超时机制。接下来，我们使用 `while let` 循环处理流的下一个事件。如果事件是 `Ok`，表示接收到了值，我们打印该值；如果事件产生了 `Err`，表示超时事件，我们打印 "Timeout occurred!"。

这样，通过 `timeout` 函数，我们可以为流添加超时机制来保护我们的程序免受长时间的等待。

