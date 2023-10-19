# File: tokio/tokio-stream/src/stream_ext/map_while.rs

在tokio-stream库中，`map_while.rs`文件中的`MapWhile`结构体提供了一个流适配器，用于在流上应用闭包并且只在闭包返回`Some`时转换元素，一旦闭包返回`None`，流将被终止。

`MapWhile`结构体是一个Stream trait的实现，用于转换输入流的元素。它由以下几个组成：

- `MapWhile<St, F>`：主结构体，其中`St`是输入流的类型，`F`是闭包函数的类型。
- `stream: St`：要转换的输入流。
- `f: F`：闭包函数，用来对流的元素进行转换。
- `pending_item: Option<PendingItem>`：用于跟踪当前转换的状态。
- `dropped`：标识流是否已经终止的标志。
- `buffer`：存储处理过程中产生的中间结果的缓冲区。

`PendingItem`结构体是一个追踪待处理的流元素的类型，它有以下几个组成：

- `item`：流中的元素。
- `pending`：流中下一个元素是否需要处理的标志。

`MapWhile`结构体实现了`FusedStream`和`Stream` trait。`FusedStream`是一个扩展了`Stream` trait的特质，表示流是否已经终止。该结构体在每个`poll_next`调用中会对输入流的元素进行转换，同时在闭包返回`None`时终止流。

简而言之，`MapWhile`结构体提供了一个转换流的适配器，在闭包函数返回`None`时终止流。它可以用于对输入流的元素进行转换和过滤。

