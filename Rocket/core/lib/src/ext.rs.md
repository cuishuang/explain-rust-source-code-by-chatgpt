# File: Rocket/core/lib/src/ext.rs

在Rocket的源代码中，`ext.rs`文件定义了一系列扩展trait和类型，用于增强Rust标准库中的一些基本类型和特性，以便更好地支持Rocket框架的功能。

下面详细介绍`ext.rs`文件中的几个重要结构：

1. `ReaderStream<R>`结构：这是一个泛型结构，用于封装任意实现了`AsyncRead`特性的类型。它实现了`Stream`特性，允许将`AsyncRead`的类型转换为可异步读取的流。

2. `Chain<T, CancellableIo<F, CancellableListener<F, Join<T>`结构：这是一个依次连接多个异步操作的结构。它将多个异步操作（比如`Future`或`Stream`）链接成一个新的异步操作。

接下来，介绍`ext.rs`文件中的几个重要Trait：

1. `AsyncReadExt`特性：这是一个扩展Trait，用于对任何实现了`AsyncRead`的类型进行扩展。它包含了一系列方法，如`async_read_exact`、`read_to_end_async`等，用于对异步读取进行快捷处理。

2. `PollExt<T>`特性：这是Rocket框架对`Poll`类型进行的扩展Trait。它包含了一系列方法，比如`poll_fn`, `map`, `map_ok`, `map_err`等，用于对`Poll`结果进行转换、条件判断、错误处理等操作。

3. `StreamExt`特性：这是Rocket框架对异步`Stream`类型进行的扩展Trait。它包含了一系列方法，如`try_for_each_concurrent`, `chunks_exact`, `buffered`等，用于对异步流进行处理、分割、缓冲等操作。

最后，介绍`ext.rs`文件中的几个枚举：

1. `State`枚举：它定义了一些可能的请求状态（请求头已读取、请求主体已读取等），以方便Rocket内部处理请求流程。

总结来说，`ext.rs`文件中定义了一系列扩展Trait和类型，用于增强Rust标准库中的一些基本类型和特性，以便更好地支持Rocket框架的功能。

