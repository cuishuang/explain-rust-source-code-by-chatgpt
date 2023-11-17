# File: vector/src/transforms/aws_ec2_metadata.rs

vector/src/transforms/aws_ec2_metadata.rs 文件中的作用是处理 AWS EC2 实例元数据。

- Ec2Metadata 结构体：表示 AWS EC2 实例元数据的主要结构。它包含实例内存储的元数据服务的基础 URL 和一个元数据客户端实例。

- Ec2MetadataTransform 结构体：Rust 生态 vector 项目中用于 AWS EC2 实例元数据的转换器。它实现了 Extract 订阅处理器，并定义了对元数据进行提取和转换的方法。

- MetadataKey 枚举：表示 AWS EC2 实例元数据的键。这是一个简单的枚举类型，包含实例元数据的各个键（如实例 ID、安全组、AMI ID 等）。

- Keys 结构体：表示一组 AWS EC2 实例元数据的键。此结构体可以将多个 MetadataKey 枚举值组合在一起，以便于处理元数据的提取和转换。

- MetadataClient 结构体：用于与 AWS EC2 实例元数据服务进行交互的客户端。它封装了发送 HTTP 请求和解析响应的逻辑，用于获取实例元数据。

- IdentityDocument 结构体：表示 AWS EC2 实例的身份验证文档。它包含实例的详细信息，如实例 ID、区域、AMI ID、VPC ID 等。

- UnexpectedHttpStatusError 结构体：表示在与元数据服务进行交互时遇到意外的 HTTP 状态码时发生的错误。它包含详细的错误消息和状态码信息。

- Ec2MetadataError 枚举：表示处理 AWS EC2 实例元数据时可能发生的错误。它包含了不同类型的错误情况，如 HTTP 请求异常、解析错误等。此枚举还提供了相应的错误消息和其他相关信息，方便进行故障排查和错误处理。

