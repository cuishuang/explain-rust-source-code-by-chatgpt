# File: tokio/tokio-stream/src/stream_ext/filter.rs

在tokio源代码中，tokio-stream库是一个提供用于操作stream的扩展的库。它包含了一系列扩展方法，比如`filter`、`map`、`then`等方法。在tokio-stream/src/stream_ext/filter.rs文件中，定义了用于过滤stream元素的相关方法和结构体。

`Filter`结构体是用于过滤stream元素的过程中保存相关状态的结构体。它具有两个泛型参数：`St`表示满足Stream trait约束的类型，`Item`表示stream元素的类型。`Filter`结构体实现了`Stream` trait，并使用封装的stream来实现相关方法。

在`Filter`中，包含了一个封装的stream（`stream: St`），通过封装，可以在不改变原始stream行为的基础上进行过滤操作。`Filter`还包含了一个闭包（`predicate`），用于对stream元素进行判断和过滤。

在`Filter`设计的实现中，定义了一系列过滤方法，比如`filter`, `filter_map`, `filter_poll`等。其中`filter`方法用于对stream进行过滤操作，`filter_map`方法不仅可以过滤 stream，还可以对元素进行转换操作，`filter_poll`方法用于获取下一个满足过滤条件的元素。

使用这些过滤方法，可以方便地对stream进行过滤，只保留满足条件的元素。可以使用自定义的闭包来指定过滤条件，根据不同的需求对stream进行过滤操作，从而获取所需的元素序列。

