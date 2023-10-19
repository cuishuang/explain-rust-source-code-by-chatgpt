# File: tokio/tokio-stream/src/stream_ext/all.rs

tokio/tokio-stream/src/stream_ext/all.rs 这个文件是 tokio-stream crate 提供的一个扩展 trait，用于添加对 Stream 的操作。

AllFuture<'a 这个结构体是一个 future，它可以将一个实现了 Stream trait 的对象转换为一个 future。它的作用是将一个 stream 中的所有元素都收集起来，并返回一个结果。当 Stream 结束时，AllFuture<'a> 会产生一个包含所有元素的 Vec<T> 或者一个错误。

All 结构体是一个实现了 Sink trait 的类型，它可以将一个 Stream 转换为一个 Sink。它的作用是将 Stream 中的元素传递给指定的 Sink，直到 Stream 结束。这在需要将 Stream 中的元素写入到 Sink 时非常有用。

AllItems 结构体是一个实现了 Sink trait 的类型，它可以将一个 Stream 转换为一个 Sink。它的作用是将 Stream 中的元素收集到一个 Vec<T> 中，并在 Stream 结束后返回这个 Vec<T>。

AllInPlace 结构体是一个实现了 Sink trait 的类型，它可以将一个 Stream 转换为一个 Sink。它的作用是将 Stream 中的元素插入到指定的 Vec<T> 中。

这些 struct 的作用是提供一些常见的操作，以便更方便地处理 Stream 类型的对象。从而使得开发者可以更加灵活和高效地处理异步流。

