# File: vector/src/sources/datadog_agent/mod.rs

在Rust生态中的vector项目中，`vector/src/sources/datadog_agent/mod.rs`文件的作用是实现与Datadog Agent API通信的相关功能。

具体来说，`DatadogAgentConfig`结构体用于存储Datadog Agent的配置信息，包括API endpoint、API key等。`ApiKeyQueryParams`结构体则是用于存储API Key的查询参数。`DatadogAgentSource`结构体是整个Datadog Agent源数据源的定义，它实现了`Source` trait并包含一些必要的方法用于获取和处理数据。`ApiKeyExtractor`结构体则是用于从HTTP请求中提取API Key的相关逻辑的定义。最后，`LogMsg`结构体是一个简单的日志消息结构，用于在Datadog Agent源数据源中存储和传递日志消息。

而`ApiError`是一个枚举类型，包含了可能的Datadog Agent API错误类型。这些错误类型包括：
- `InvalidApiKey`: 无效的API Key错误
- `UnexpectedResponse`: 不符合预期的API响应
- `NetworkError`: 网络错误
- `RateLimitExceeded`: 超过API请求限制
- `Unknown`: 未知的错误

这些枚举项提供了不同类型的错误信息，以供源代码中根据具体情况来处理错误时进行选择和匹配。

以上就是`vector/src/sources/datadog_agent/mod.rs`文件中包含的几个结构体和枚举类型的详细介绍。这些结构体和枚举类型的作用是为了实现与Datadog Agent API的交互以及处理相关的错误情况。

