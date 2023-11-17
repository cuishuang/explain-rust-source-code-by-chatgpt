# File: vector/src/sinks/datadog/traces/apm_stats/flusher.rs

在Rust生态vector项目中，sinks文件夹是用于处理数据发送的目标（sinks）的地方。在sinks/datadog/traces/apm_stats文件夹下，flusher.rs文件的作用是实现数据的刷新，将统计信息发送到Datadog APM（Application Performance Monitoring）。

在该文件中，定义了一个名为`Flusher`的struct，它负责刷新（flush）收集到的统计信息。具体来说，Flusher在初始化时会获取一个线程池以进行异步处理，它维护一个等待发送的消息队列。当有新的统计信息到来时，Flusher会将其添加到队列中，并安排后台线程从队列中获取并发送到Datadog APM。

`ApmStatsSender`是在sinks/datadog/traces/apm_stats/mod.rs文件中定义的。这个struct是一个通过Rust的`crossbeam_channel`实现的消息发送器。它负责将统计信息发送到Flusher，并在必要时进行同步。

总结一下，文件flusher.rs的作用是定义了一个刷新器（Flusher），用于异步刷新收集到的统计信息，并将其发送到Datadog APM。而ApmStatsSender是一个消息发送器，用于将统计信息发送到刷新器。

