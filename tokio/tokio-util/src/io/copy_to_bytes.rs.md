# File: tokio/tokio-util/src/io/copy_to_bytes.rs

`tokio-util/src/io/copy_to_bytes.rs` 文件中定义了 `CopyToBytes` trait 和其实现，用于将数据从 `AsyncRead` 的实现类型复制到字节数组中。

`CopyToBytes` 是一个 trait，它定义了一个方法 `async fn copy_to_bytes(self) -> io::Result<Vec<u8>>`，这个方法接收一个实现了 `AsyncRead` 的对象 `self`，并返回一个包含读取的字节的 `Vec<u8>`。

`CopyToBytes` trait 的实现提供了一种将异步读取的数据复制到字节数组的方法。它首先创建一个容量为 4096 字节的缓冲区，然后循环读取数据并调整缓冲区的大小以适应读取的数据大小。最后返回包含读取数据的字节数组。

`CopyToBytes` trait 的几个 struct 类型参数的作用如下：
1. `S` 是实现了 `AsyncRead` 的类型，它表示要从该类型中读取数据并复制到字节数组中。
2. `T` 是 `Sink` 的类型，它实现了 `Sink<Item = Bytes>` trait。在读取数据时，将每个读取的字节包装成 `Bytes` 类型，并发送到该 `Sink` 中。

这些 struct 的作用是将数据从 `AsyncRead` 的实现类型复制到字节数组，并通过 `Sink` 发送它们。

