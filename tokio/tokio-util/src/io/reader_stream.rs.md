# File: tokio/tokio-util/src/io/reader_stream.rs

tokio-util是Tokio库的一个辅助工具模块，其中的io/reader_stream.rs文件提供了一个ReaderStream结构体和一些相关的方法，用于将一个实现了AsyncRead trait的类型封装成一个异步流。

ReaderStream结构体的定义如下：
```rust
pub struct ReaderStream<R> {
    inner: R,
    buf: Vec<u8>,
    pos: usize,
}
```

该结构体有三个字段：
- `inner`：一个实现了AsyncRead trait的类型，表示底层的异步读取器。
- `buf`：一个用于缓存读取数据的字节数组。
- `pos`：记录当前在buf中的读取位置。

ReaderStream结构体实现了Stream trait，可以被用作异步流。它通过不断调用inner的poll_read方法来获取数据并推送到流中。当一次读取操作完成后，会检查是否读取到了EOF（文件结束符），如果没有，则将继续读取并推送数据。

ReaderStream还实现了AsyncRead trait，使得它自身也能被当作异步读取器使用。通过实现这个trait，可以方便地组合和复用ReaderStream。

除了ReaderStream结构体外，该文件还提供了一些相关方法，如：
- `read`：从底层的异步读取器读取指定的字节数目到buf中，返回一个Future。
- `poll_read`：异步地从底层的异步读取器读取指定字节数目到buf中，返回一个Poll结果。

这些方法是ReaderStream的核心方法，用于实现从底层读取器异步读取数据并推送到流中的功能。

总的来说，tokio-util/io/reader_stream.rs文件中的ReaderStream结构体和相关方法提供了一种将实现了AsyncRead trait的类型封装成异步流的方式，并且能够异步地从底层读取器中读取数据。这样，使用者可以更方便地操作异步读取器和流。

