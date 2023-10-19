# File: tokio/tokio/src/io/util/read_until.rs

在tokio的源代码中，`tokio/src/io/util/read_until.rs`文件是用于读取数据直到遇到指定分隔符的操作。该文件是tokio库中实现了`ReadUntil` trait的具体实现。

`ReadUntil` trait是tokio库中的一个异步读取数据的trait，它定义了`read_until`方法，该方法用于从实现了`AsyncRead` trait的类型中异步读取数据，并在遇到指定的分隔符时停止读取。

`ReadUntil` trait的具体实现位于`ReadUntil` struct中。`ReadUntil` struct定义了一个异步read_until方法，该方法接受一个实现了`AsyncRead` trait的类型和一个u8类型的分隔符作为参数，返回一个包含读取数据的future。

`IntoInner`结构体是一个为`ReadUntil`结构体定义的辅助结构体。它负责将`ReadUntil`包装的`AsyncRead`类型恢复为原始类型。

`ReadUntil<'a, R>`是一个异步读取数据的包装器，其中`'a`代表的是它的生命周期参数。它实现了`AsyncRead` trait，可以用于在tokio运行时中进行异步读取数据操作。一般情况下，需要将要读取的数据和一个分隔符传递给`ReadUntil`结构体的构造函数，然后使用`AsyncRead` trait提供的方法进行异步读取。

