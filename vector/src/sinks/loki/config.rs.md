# File: vector/src/sinks/loki/config.rs

在Rust生态的Vector项目中，`vector/src/sinks/loki/config.rs`文件的作用是定义与Loki日志服务相关的配置。

首先，`LokiConfig`结构体定义了与Loki日志服务连接相关的配置选项。它包含以下字段：

- `batch`字段表示数据批处理的设置，是一个`LokiBatchSettings`结构体类型。
- `request_timeout_secs`字段表示发送请求超时时间的秒数。
- `token`字段是用于访问Loki服务的身份验证令牌。
- `allowed_incomplete`字段表示是否允许发送不完整的数据。
- `compression`字段表示数据压缩的设置，是一个`LokiCompression`枚举类型。
- `url`字段表示Loki服务的URL地址。

接下来，`LokiDefaultBatchSettings`结构体定义了与数据批处理相关的默认设置。它包含以下字段：

- `batch_size`字段表示每个批处理请求中的最大事件数量。
- `batch_timeout`字段表示触发批处理的超时时间。

`ExtendedCompression`枚举定义了压缩数据的不同选项。它包含以下成员：

- `Inline`表示以行为单位进行压缩。
- `LatLng`表示将经纬度数据压缩为一个单独的字段。
- `Content`表示将内容字段压缩为一个单独的字段。

`CompressionConfigAdapter`枚举定义了Loki压缩配置的适配器。它包含以下成员：

- `None`表示不压缩数据。
- `Gzip`表示使用Gzip算法压缩数据。

`OutOfOrderAction`枚举定义了处理乱序事件的不同选项。它包含以下成员：

- `Block`表示阻塞事件处理直到事件到达正确的顺序。
- `Drop`表示丢弃乱序事件。
- `Continue`表示继续处理乱序事件，但会给出警告。

通过以上的配置选项和枚举类型，可以灵活地配置和控制将日志数据发送到Loki日志服务的行为。

