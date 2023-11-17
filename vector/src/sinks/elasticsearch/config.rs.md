# File: vector/src/sinks/elasticsearch/config.rs

在Rust生态中，vector是一个用于数据收集和路由的高性能日志代理。vector通过插件化的方式支持将数据从不同的数据源发送到各种目标。在vector的源代码中，`vector/src/sinks/elasticsearch/config.rs`文件的作用是定义了与elasticsearch相关的配置信息。

在该文件中，有三个结构体定义：`ElasticsearchConfig`、`BulkConfig`和`DataStreamConfig`。

`ElasticsearchConfig`结构体定义了与elasticsearch相关的配置选项，包括elasticsearch主机地址、端口、索引名称等。它包含以下字段：

- `hosts`: 指定elasticsearch主机地址和端口列表。
- `index`: 指定要发送到的elasticsearch索引的名称。
- `doc_type`: 指定要发送的文档类型。
- `id_key`: 指定要用作文档ID的字段。
- `request_size`: 指定在批量请求中发送的最大事件数。

`BulkConfig`结构体定义了elasticsearch的批量请求配置选项，用于控制将多个事件一次性发送给elasticsearch的行为。它包含以下字段：

- `batch_size`: 指定要在单个批量请求中发送的最大事件数。
- `batch_timeout`: 指定在超时之前等待发送批量请求的最长时间。

`DataStreamConfig`结构体定义了与elasticsearch数据流相关的配置选项，用于将数据流写入elasticsearch。它包含以下字段：

- `index_template_name`: 指定数据流的索引模板名称。
- `index_template_pattern`: 指定数据流的索引模板匹配模式。
- `ilm_enabled`: 指示是否启用索引生命周期管理。
- `ilm_policy_name`: 指定索引生命周期管理策略名称。

这些结构体定义了vector与elasticsearch之间的配置关系，使得用户可以根据需求进行相应的配置，例如指定elasticsearch主机地址和索引名称，控制事件发送的批量大小和超时时间，以及配置索引生命周期管理等。这些配置选项使得用户能够灵活地使用vector将数据发送到elasticsearch，并根据具体需求进行相应的调整。

