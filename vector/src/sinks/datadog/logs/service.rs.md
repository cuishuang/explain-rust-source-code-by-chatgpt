# File: vector/src/sinks/datadog/logs/service.rs

在Rust生态vector项目的源代码中，vector/src/sinks/datadog/logs/service.rs是用于实现Datadog日志API服务的文件。

该文件定义了一些关键的结构体和实现，其中包括：

1. LogApiRetry：该结构体用于指定重试策略，当发送日志API请求失败时进行重试。
   - 字段：
     - `limit`：最大重试次数
     - `initial_backoff_secs`：初始重试间隔时间
     - `max_backoff_secs`：最大重试间隔时间
     - `new_errors_fraction`：增加重试等待时间的错误比例

2. LogApiRequest：该结构体表示向Datadog日志API发送的请求。
   - 字段：
     - `logs`：日志事件列表
     - `ddsource`：指定日志的来源
     - `ddtags`：指定日志的标签
     - `service`：指定日志的服务

3. LogApiResponse：该结构体表示从Datadog日志API接收到的响应。
   - 字段：
     - `status_code`：响应的HTTP状态码
     - `message`：响应消息

4. LogApiService：该结构体实现了Datadog日志API服务的相关功能。
   - 方法：
     - `new`：创建一个新的LogApiService实例
     - `send_logs`：发送日志到Datadog日志API
     - `build_request`：构建发送日志请求
     - `parse_response`：解析接收到的API响应
     - `should_retry`：检查是否应该进行重试

总的来说，`LogApiService`是一个用于与Datadog日志API进行交互的工具，它提供了发送日志、构建请求、解析响应和检查重试等功能。`LogApiRetry`用于指定重试策略，`LogApiRequest`表示发送的请求，`LogApiResponse`表示接收的响应。这些结构体共同协同工作，实现了对Datadog日志API的集成。

