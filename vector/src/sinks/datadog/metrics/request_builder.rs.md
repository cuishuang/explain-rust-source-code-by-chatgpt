# File: vector/src/sinks/datadog/metrics/request_builder.rs

在Rust生态的vector项目中，`vector/src/sinks/datadog/metrics/request_builder.rs`文件的作用是定义与Datadog指标发送相关的请求构建器。

`DDMetricsMetadata`结构体用于存储Datadog指标数据的元数据，包括指标名称、标签等信息。它实现了`From<Record>`和`From<&Record>`来将Record类型的数据转换为Datadog指标的元数据。

`DatadogMetricsRequestBuilder`结构体是一个请求构建器，它用于构建发送到Datadog的指标数据请求。它包含Datadog相关的配置信息以及Datadog的API密钥。通过`DatadogMetricsRequestBuilder`，您可以配置要发送的指标数据、设置每个请求的元数据、设置请求的超时时间等。

`RequestBuilderError`是一个枚举类型，它定义了请求构建器的错误类型。它包括以下几种错误：

- `InvalidBaseUrl`: 当提供的Datadog基本URL无效时，会发生此错误。
- `MissingApiToken`: 如果API密钥未提供，则会发生此错误。
- `InvalidDatadogHost`: 当提供的Datadog主机名无效时，会发生此错误。
- `InvalidTimeout`: 如果提供的超时时间无效，则会发生此错误。
- `InvalidMethod`: 当提供的请求方法无效时，会发生此错误。

这些结构体和枚举类型一起为构建和处理Datadog指标发送请求提供了基本功能，并提供了错误处理机制。

