# File: vector/src/sinks/pulsar/config.rs

在Rust生态vector项目的源代码中，`vector/src/sinks/pulsar/config.rs` 文件的作用是定义 Pulsar sink 的配置。

具体来说，该文件中定义了以下几个结构体：

1. `PulsarSinkConfig`: 定义了 Pulsar sink 的基本配置，包括 Pulsar 服务的地址、主题名称、超时时间等。
2. `PulsarBatchConfig`: 定义了 Pulsar sink 的批处理配置，包括批处理的最大大小和最大等待时间。
3. `PulsarAuthConfig`: 定义了 Pulsar sink 的验证配置，包括其中的认证类型、访问令牌等。
4. `OAuth2Config`: 定义了 OAuth2 授权配置，包括认证和访问令牌服务器的 URL、客户端 ID 和客户端密钥等。

此外，该文件还定义了一个枚举类型 `PulsarCompression`，用于配置 Pulsar sink 的压缩类型。该枚举包括以下几个成员：

1. `None`: 表示不使用压缩。
2. `LZ4`: 表示使用 LZ4 压缩算法。
3. `ZLib`: 表示使用 ZLib 压缩算法。
4. `ZStd`: 表示使用 ZStd 压缩算法。

这些配置结构体和枚举类型的目的是允许用户在使用 Pulsar sink 时根据需要进行灵活的配置，包括连接到 Pulsar 服务的参数、数据压缩类型、认证方式等。通过这些配置，使用者可以根据实际情况进行适配和优化，以满足其具体需求。

