# File: vector/src/api/schema/metrics/sent_bytes.rs

在Rust生态vector项目中，`vector/src/api/schema/metrics/sent_bytes.rs` 文件的作用是定义一些与发送字节数相关的指标和结构体。

`SentBytesTotal` 结构体代表了一个名为 `SentBytesTotal` 的指标，它包含了关于发送字节数的一些统计信息。这个结构体用于表示在一段时间内发送的总字节数，并提供了一些方法来更新和访问这个指标。例如，可以使用 `SentBytesTotal::new()` 创建一个新的 `SentBytesTotal` 实例，并使用 `SentBytesTotal::increment()` 方法来增加发送字节数。

`ComponentSentBytesTotal` 结构体代表了组件级别的发送字节数指标。每个组件都可以有自己的 `ComponentSentBytesTotal` 实例，用于跟踪该组件在一段时间内发送的总字节数。该结构体通常用于在组件级别收集发送字节数数据，并提供了一些方法来更新和访问这个指标。例如，可以使用 `ComponentSentBytesTotal::new()` 创建一个新的 `ComponentSentBytesTotal` 实例，并使用 `ComponentSentBytesTotal::increment()` 方法来增加组件的发送字节数。

`ComponentSentBytesThroughput` 结构体代表了组件级别的发送字节吞吐量指标。与 `ComponentSentBytesTotal` 不同，`ComponentSentBytesThroughput` 关注的是单位时间内的发送字节数，即吞吐量。该结构体包含了一些字段，例如 `bytes_per_second`，用于存储每秒发送的字节数，并提供了一些方法来更新和访问这个指标。例如，可以使用 `ComponentSentBytesThroughput::new()` 创建一个新的 `ComponentSentBytesThroughput` 实例，并使用 `ComponentSentBytesThroughput::update()` 方法来更新组件的发送字节吞吐量。

这些指标和结构体用于记录和管理发送字节数的相关数据，可以帮助开发者了解系统的性能和发送行为。他们提供了一种统一的方式来收集、存储和访问发送字节数的信息，从而方便进行性能分析和监控。

