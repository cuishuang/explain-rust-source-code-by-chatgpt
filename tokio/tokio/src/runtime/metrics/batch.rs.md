# File: tokio/tokio/src/runtime/metrics/batch.rs

tokio/tokio/src/runtime/metrics/batch.rs 是 Tokio 运行时库中的一个文件，它主要定义了与度量相关的结构体和相关的功能实现。

MetricsBatch 结构体是用来收集和批处理度量事件的，它拥有一个 Vec<MetricsEvent> 的 buffer，当度量事件到达时，可以将其添加到 buffer 中。MetricsBatch 还实现了度量事件的批处理逻辑，即当 buffer 达到一定大小或者经过一段时间后，就会执行批处理操作。在批处理操作中，可以将多个度量事件发送到度量报告器（Metric Reporter）中。

PollTimer 结构体用于度量度量上一个轮询的时间间隔，即度量事件的频率。它记录了上一次度量事件被执行的时间，当需要收集度量时，可以使用它来判断是否需要执行度量操作。

同时，这个文件中还定义了一些相关的实用函数和类型别名，用于度量过程中的计时、记录错误等。

总的来说，tokio/tokio/src/runtime/metrics/batch.rs 提供了度量数据的批处理和调度的功能，以及度量事件的频率控制，方便开发者监控和调优 Tokio 程序的性能和运行状况。

