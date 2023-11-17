# File: vector/src/sinks/elasticsearch/request_builder.rs

文件`request_builder.rs`实现了一个用于构建Elasticsearch请求的构建器（builder）模式的结构体`ElasticsearchRequestBuilder`。该结构体提供了一系列方法，用于设置请求的各种参数，并最终构建一个Elasticsearch请求。

具体来说，`ElasticsearchRequestBuilder`结构体有以下主要功能：

1. `new`方法：创建一个新的`ElasticsearchRequestBuilder`实例。
2. `index`、`type_`、`id`等方法：用于设置请求的索引、类型和文档ID等关键参数。
3. `method`方法：设置请求的HTTP方法（如GET、POST、DELETE等）。
4. `body`方法：设置请求的内容体（如文档内容）。
5. `routing`、`parent`等方法：设置请求的路由、父文档等参数。
6. `metadata`方法：设置请求的元数据，包括超时时间、刷新策略等。
7. `send`方法：将构建的请求发送到Elasticsearch服务器，并返回一个结果类型（`Result`）。
8. `extract_response`方法：从HTTP响应中提取出Elasticsearch响应。

`ElasticsearchRequestBuilder`结构体内嵌了一个`Metadata`结构体，用于存储请求的元数据。`Metadata`结构体具有以下作用：

1. `timeout`字段：存储请求超时时间。
2. `refresh`字段：存储请求的刷新策略。
3. `version`字段：存储请求的版本信息。

通过使用`ElasticsearchRequestBuilder`结构体，可以方便地构建出具有特定参数的Elasticsearch请求，并将其发送到Elasticsearch服务器上进行处理。

