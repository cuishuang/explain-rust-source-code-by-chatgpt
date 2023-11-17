# File: vector/src/sinks/appsignal/config.rs

在Rust生态的vector项目中，vector/src/sinks/appsignal/config.rs文件的作用是定义了与AppSignal集成相关的配置选项。

AppsignalConfig结构体定义了传递给AppSignal发送器的配置参数。它包含了以下字段：

- `endpoint`: 表示AppSignal的传输协议和地址，例如："https://push.appsignal.com"
- `api_key`: 表示AppSignal的API密钥，用于在发送数据时进行身份验证
- `abort_on_decorator_failure`: 表示在发送期间，是否在处理数据时出现错误时立即终止

AppsignalDefaultBatchSettings结构体定义了AppSignal发送器的默认批处理设置。它包含了以下字段：

- `max_size_bytes`: 定义批处理的最大大小（以字节为单位），例如：1024 * 1024，表示1MB
- `timeout_secs`: 定义批处理的超时时间（以秒为单位），例如：5，表示5秒
- `max_events`: 定义批处理中包含的最大事件数量

这些结构体的作用是在集成AppSignal时，提供了配置选项和默认的批处理设置，以便用户可以根据自己的需求对向AppSignal发送数据的行为进行自定义调整。通过修改这些结构体中的字段值，用户可以更改AppSignal的传输地址、API密钥、批处理的大小和超时时间等参数。

