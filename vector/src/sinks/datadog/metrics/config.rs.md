# File: vector/src/sinks/datadog/metrics/config.rs

vector/src/sinks/datadog/metrics/config.rs是Rust生态中的vector项目中的一个文件，它定义了与Datadog Metrics相关的配置。

1. DatadogMetricsDefaultBatchSettings: 这个struct定义了Datadog Metrics的默认批处理设置。它包含了用于指定批处理大小，最大延迟时间以及最大批处理时间的字段。

2. DatadogMetricsEndpointConfiguration: 这个struct定义了Datadog Metrics的端点配置，包含了用于指定目标Datadog Metrics的主机地址和端口号的字段。

3. DatadogMetricsConfig: 这个struct是整个Datadog Metrics的配置结构，它包含了DatadogMetricsDefaultBatchSettings和DatadogMetricsEndpointConfiguration，用于配置Datadog Metrics的批处理设置和端点配置。

4. SeriesApiVersion: 这个enum定义了Series API的版本。它有两个具体的值：V1和V2，用于指示使用的是哪个版本的Series API。

5. DatadogMetricsEndpoint: 这个enum定义了Datadog Metrics的端点。它有两个具体的值：Http和Udp，用于指示使用的是哪种方式发送数据到Datadog Metrics的端点。

这些struct和enum的作用是将Datadog Metrics的配置信息进行结构化管理，使得用户可以更方便地配置Vector与Datadog Metrics的集成，并提供了一些默认值供用户使用。

