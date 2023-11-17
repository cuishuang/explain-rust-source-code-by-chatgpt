# File: vector/src/sinks/datadog/mod.rs

在Rust生态vector项目的源代码中，vector/src/sinks/datadog/mod.rs文件的作用是实现了将数据发送到Datadog的sink。

首先，DatadogCommonConfig这几个struct用于配置Datadog sink的一些常用参数。这些struct包括：
1. `DatadogRateLimitConfig`：用于配置Datadog的速率限制参数，如每秒发送的请求数量。
2. `DatadogTimeoutConfig`：用于配置发送请求时的超时参数，包括连接超时和请求超时。
3. `DatadogBatchEventsConfig`：用于配置发送批量事件的参数，包括批量发送的最大事件数量和最大请求大小。
4. `DatadogCommonConfig`：是上述配置的一个组合，用于方便地管理和传递这些配置参数。

接下来，DatadogApiError这几个enum定义了Datadog API可能返回的错误类型。这些enum包括：
1. `DatadogApiError::RateLimited`：表示请求因达到速率限制而被拒绝。
2. `DatadogApiError::Unauthorized`：表示请求未授权，通常是由于提供的API密钥无效。
3. `DatadogApiError::BadRequest`：表示请求格式不正确或参数错误。
4. `DatadogApiError::InternalServerError`：表示服务器内部错误。
5. `DatadogApiError::Unknown`：用于表示未知的API错误类型。

这些Enum用于在处理Datadog API返回的错误时提供清晰的错误类型分类和处理。

总而言之，vector/src/sinks/datadog/mod.rs是实现将数据发送到Datadog的sink的代码文件。其中，DatadogCommonConfig struct用于配置Datadog sink的参数，DatadogApiError enum用于处理Datadog API返回的错误。

