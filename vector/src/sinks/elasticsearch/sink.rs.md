# File: vector/src/sinks/elasticsearch/sink.rs

在Rust生态vector项目的源代码中，`vector/src/sinks/elasticsearch/sink.rs`文件的作用是实现了将事件流发送到 Elasticsearch 的功能。

首先，文件中定义了一个 `PartitionKey` 结构体，用于表示 Elasticsearch 的分区键。它是一个存储字符串的简单的包装类型，用于指定分区的值。

接下来，文件中定义了一个泛型结构体 `ElasticsearchSink<S>`，其中 `S` 是一个泛型参数，表示存储的状态。`ElasticsearchSink` 结构体实现了 `Sink` Trait，可以被 vector 使用作为一个目的地，用于将事件发送到 Elasticsearch。

`ElasticsearchSink` 结构体包含了以下字段：
- `http`: `HttpClient` 的实例，用于发送 HTTP 请求到 Elasticsearch。
- `uri`: Elasticsearch 的 URI。
- `index`: Elasticsearch 的索引名称。
- `id_field`: 用于指定事件字段中表示文档 ID 的字段。
- `batch_size`: 用于指定每个批处理请求的事件数量。
- `doc_type`: 用于指定文档类型，已在 Elasticsearch 7.x 版本中弃用。
- `timeout`: 用于指定 HTTP 请求的超时时间。

`ElasticsearchSink` 结构体实现了 `ElasticsearchSink` trait 中定义的方法：
- `new`: 根据给定的参数创建一个新的 `ElasticsearchSink` 实例。
- `set_basic_auth`: 设置用于身份验证的基本认证信息。
- `start`: 初始化 Elasticsearch 索引和文档类型。
- `retries_until_up`: 返回 Vector 尝试重新连接到 Elasticsearch 的次数。
- `request_send`: 发送事件到 Elasticsearch。
- `construct_uri`: 构造 Elasticsearch 请求的 URI。
- `upsert_doc`: 如果存在指定 ID 的文档，则更新它；否则创建一个新的文档。
- `build_upsert`: 构建一个“upsert”请求的 JSON 体。
- `http_send`: 发送 HTTP 请求到 Elasticsearch。

总之，`vector/src/sinks/elasticsearch/sink.rs`文件中的 `ElasticsearchSink` 结构体实现了将事件流发送到 Elasticsearch 的功能，并封装了与 Elasticsearch 交互的逻辑。`PartitionKey` 结构体则用于表示 Elasticsearch 的分区键。

