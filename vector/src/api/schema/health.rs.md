# File: vector/src/api/schema/health.rs

在Rust生态中，vector 项目是一个开源的数据收集、转换和路由工具。在该项目中，`vector/src/api/schema/health.rs` 文件是用于定义与健康检查相关的 API 端点的。

具体来说，`health.rs` 文件中定义了以下三个 struct：

1. `Heartbeat`：这个 struct 代表心跳检查。在 API 端点中，可以通过发送一个心跳检查来确认服务的存活状态。它包含一个字段 `status`，用于表示心跳检查的状态。

2. `HealthQuery`：这个 struct 代表健康检查查询。它用于在 API 端点中提供健康检查的功能。它包含一个字段 `subscribed`，表示是否订阅了健康检查结果。

3. `HealthSubscription`：这个 struct 代表健康检查的订阅。它用于订阅健康检查的结果，以便实时监控服务的健康状态。它包含一个字段 `status`，用于表示健康检查的状态。

这些 struct 的作用是提供了在 vector 项目中进行健康检查的功能。通过使用这些 struct，用户可以发送心跳检查来确认服务是否存活，查询健康检查的结果，并实时订阅健康检查的状态变化。这对于监控和确保系统的正常运行非常重要。

