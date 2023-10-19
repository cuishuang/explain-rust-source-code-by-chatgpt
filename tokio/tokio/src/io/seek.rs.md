# File: tokio/tokio/src/io/seek.rs

在tokio源代码中，tokio/tokio/src/io/seek.rs这个文件的作用是提供异步IO操作时进行seek操作的功能。

在该文件中，包含了三个主要的结构体，分别是:

1. `Seek`：是一个 trait，用于定义异步seek操作的方法。它是tokio库为实现异步io流处理而提供的标准 trait 之一。这个trait主要定义了异步seek操作的方法，包括`poll_seek`和`start_seek`。

2. `SeekFromFuture`：是一个实现了`Future` trait的结构体，用于表示异步seek操作的状态。它包含一个`stream`字段，用于存储异步IO流，以及一个`pos`字段，用于表示所需seek的位置。`SeekFromFuture`实现了`poll`方法来处理seek操作的异步逻辑。具体来说，它会通过调用`stream.start_seek`方法返回一个`Future`,并将其包装为`Pin<Box<dyn Future<Output = Result<u64, io::Error>>>>`类型的对象。在`start_seek`的实现中，它会检查是否需要进行seek操作，然后调用实际的seek操作方法，并返回`Future`来表示操作的完成。

3. `SeekFrom`：是一个表示偏移量的枚举类型。它有两个变体，分别是`Start`和`Current`，用于表示相对于文件开头和当前位置的偏移量。这个类型实现了`From` trait，可以方便地从`std::io::SeekFrom`类型转换。

总而言之，tokio/tokio/src/io/seek.rs文件提供了异步IO操作的seek功能的实现，包括定义了`Seek` trait，实现了`SeekFromFuture`结构体和`SeekFrom`枚举类型，这些都是为了支持在异步IO流中进行seek操作。

