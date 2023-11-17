# File: vector/src/sinks/elasticsearch/retry.rs

在Rust生态的vector项目中，`vector/src/sinks/elasticsearch/retry.rs`文件的作用是定义了与Elasticsearch连接失败时的重试逻辑。

该文件中定义了以下几个结构体：

1. `EsResultResponse`: 该结构体表示Elasticsearch请求结果的响应信息。它包含请求的状态码、消息和结果。

2. `EsIndexResult`: 表示Elasticsearch索引操作的结果信息。它包含了操作的索引名称、状态码和结果。

3. `EsErrorDetails`: 该结构体包含了Elasticsearch错误的详细信息，包括错误类型、错误原因和错误堆栈等。

4. `ElasticsearchRetryLogic`: 该结构体定义了Elasticsearch连接失败时的重试逻辑。它包含了最大重试次数、重试间隔和是否启用重试等配置信息。

此外，还定义了以下几个枚举类型：

1. `EsResultItem`: 该枚举类型表示Elasticsearch请求结果的每个条目的结果状态。它可以是成功、重试、失败或超时等。

以上就是`vector/src/sinks/elasticsearch/retry.rs`文件中定义的几个结构体和枚举类型的作用。它们为连接失败时的重试提供了相关的处理逻辑和数据结构。

