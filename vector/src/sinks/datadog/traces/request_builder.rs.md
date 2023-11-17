# File: vector/src/sinks/datadog/traces/request_builder.rs

在Rust生态中的vector项目中，`request_builder.rs`文件位于`vector/src/sinks/datadog/traces/`目录下，其主要作用是构建发送到Datadog的跟踪数据请求。

`DatadogTracesRequestBuilder`结构体是负责构建Datadog的跟踪数据请求的请求体。它包含了请求的URL、请求头、请求参数等信息，以及跟踪数据的序列化后的JSON表示。

`DDTracesMetadata`结构体用于存储Datadog跟踪数据请求的元数据，包括API密钥、应用程序ID等。

`RequestBuilderError`枚举类型用于表示请求构建的错误。它包括多个成员变量，如`InvalidUrl`（URL无效）、`InvalidApiKey`（API密钥无效）等，可以根据具体错误情况进行错误处理。

总体而言，`request_builder.rs`文件中的`DatadogTracesRequestBuilder`结构体和相关的数据类型是用于构建和处理发送到Datadog的跟踪数据请求的。

