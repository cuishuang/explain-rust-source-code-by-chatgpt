# File: vector/src/api/schema/metrics/output.rs

在Rust生态vector项目的源代码中，`vector/src/api/schema/metrics/output.rs`文件的作用是定义输出指标的数据结构和相关实现。

该文件中的`Output`结构体定义了输出指标的数据结构。它包含了一系列字段，其中比较重要的字段如下：

- `received`：表示输出接收的事件数量。
- `processed`：表示输出成功处理的事件数量。
- `errored`：表示输出处理过程中发生错误的事件数量。
- `throughput`：表示输出的吞吐量，即每秒钟处理的事件数量。

此外，`Output`结构体还实现了一些方法，例如`new`方法用于创建一个新的`Output`实例，`clear_error`方法用于清除错误计数。

另外，`OutputThroughput`结构体是`Output`的一个衍生结构体，它是为了方便输出指标吞吐量的计算而引入的。它包含了一个时间戳和相应的计数。当输出指标吞吐量需要计算时，可以使用`OutputThroughput`结构体来表示特定时间段内的处理数量。

总而言之，`vector/src/api/schema/metrics/output.rs`文件定义了输出指标相关的数据结构和方法，帮助开发者跟踪和统计输出事件的处理情况，并在需要时计算输出指标的吞吐量。

