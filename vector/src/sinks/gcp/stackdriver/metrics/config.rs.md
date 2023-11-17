# File: vector/src/sinks/gcp/stackdriver/metrics/config.rs

在Rust生态的Vector项目中，`vector/src/sinks/gcp/stackdriver/metrics/config.rs` 文件的作用是定义了与 Stackdriver Metrics 相关的配置和请求构建器。

下面是对每个 struct 的详细介绍：

1. `StackdriverConfig`: 这个 struct 定义了 Stackdriver Metrics 的配置项，包括 `project_id`（用于指定要写入的 GCP 项目 ID）、`credentials`（用于 Stackdriver 认证的凭证信息）等。

2. `StackdriverMetricsDefaultBatchSettings`: 这个 struct 定义了 Stackdriver Metrics 默认的批处理设置，包括 `max_count`（批处理的最大指标数量，默认为 1000）、`max_size_bytes`（批处理的最大大小限制，默认为 1024 * 1024 字节）等。

3. `StackdriverMetricsServiceRequestBuilder`: 这个 struct 是用于构建 Stackdriver Metrics 请求的构建器。它包含了多个字段，用于设置请求的各种细节，例如 `project_id`、`time_series`（用于指定要写入的时间序列数据）等。它提供了一系列的方法，用于设置每个字段的值，并最终构建出一个有效的请求对象。

通过使用 `StackdriverConfig`、`StackdriverMetricsDefaultBatchSettings` 和 `StackdriverMetricsServiceRequestBuilder` 这些 struct，Vector 可以在与 Stackdriver Metrics 进行通信时提供更灵活的配置和请求构建能力。

