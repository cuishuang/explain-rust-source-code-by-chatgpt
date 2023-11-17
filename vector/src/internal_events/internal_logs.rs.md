# File: vector/src/internal_events/internal_logs.rs

在Rust生态vector项目中，`vector/src/internal_events/internal_logs.rs`文件的作用是定义内部日志相关的事件结构和处理方法。此文件实现了`InternalLogsBytesReceived`和`InternalLogsEventsReceived`这两个结构体。

`InternalLogsBytesReceived`结构体表示接收到的内部日志的字节数。它包含以下字段：
- `timestamp`: 表示接收到日志的时间戳。
- `bytes`: 表示接收到的字节数。

`InternalLogsEventsReceived`结构体表示接收到的内部日志事件的数量。它包含以下字段：
- `timestamp`: 表示接收到日志的时间戳。
- `events`: 表示接收到的事件数量。

这两个结构体的作用是跟踪和统计内部日志的字节数和事件数量。它们可以在代码的不同部分被触发和使用，以帮助开发者了解系统中内部日志的流量和处理情况。这些结构体可以在内部日志模块的其他组件中被订阅和处理，从而进行相应的日志记录和分析。

