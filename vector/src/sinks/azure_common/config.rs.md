# File: vector/src/sinks/azure_common/config.rs

在Rust生态的Vector项目中，`vector/src/sinks/azure_common/config.rs`文件的作用是定义与 Azure Blob 存储相关的配置项。

该文件中定义了以下几个结构体和枚举类型：

1. `AzureBlobRequest` 结构体：表示与 Azure Blob 存储进行交互时的请求信息。它包含了请求的方法、路径、请求头等信息。

2. `AzureBlobMetadata` 结构体：表示 Azure Blob 存储中的一个 Blob 对象的元数据。它包含了 Blob 的名称、大小、最后修改时间等信息。

3. `AzureBlobRetryLogic` 结构体：表示 Azure Blob 存储的失败重试逻辑。它包含了重试的次数、重试的延迟等配置信息。

4. `AzureBlobResponse` 结构体：表示与 Azure Blob 存储进行交互后的响应信息。它包含了响应的状态码、响应头、响应体等信息。

5. `HealthcheckError` 枚举类型：表示健康检查时可能出现的错误情况。它包含了诸如连接错误、授权错误、请求超时等错误类型。

这些结构体和枚举类型在 `vector/src/sinks/azure_common/config.rs` 文件中的定义，用于封装 Azure Blob 存储相关的配置和数据，并提供给 Vector 项目的其他模块使用。

