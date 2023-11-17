# File: vector/src/sinks/azure_common/sink.rs

在Rust生态中，vector项目是一个高性能、可扩展的日志收集工具。在vector的代码库中，vector/src/sinks/azure_common/sink.rs文件的作用是实现了将日志数据推送到 Azure Blob 存储的功能。

在该文件中，有一个名为AzureBlobSink<Svc>的结构体。该结构体是vector的Azure Blob存储的 Sink（下沉）类型之一，用于将日志数据写入Azure Blob存储。这个结构体实现了Sink trait，因此可以使用vector的一致性接口进行数据收集和处理。

AzureBlobSink<Svc>结构体中有以下主要成员：
1. `retry_limit`: 表示在遇到错误时重试操作的最大次数。
2. `service`: Azure Blob存储的服务实例，用于与Azure Blob API进行交互。
3. `container_name`: Azure Blob存储中的容器名称，表示要将日志数据写入的容器。
4. `blob_name_format`: 表示生成Azure Blob存储中每个Blob名称的格式。
5. `content_encoding`: 表示要使用的数据编码格式，如gzip等。

另外还有一个辅助结构体`AzureBlobSvc`，用于与Azure Blob的低级API进行交互。它包含以下主要成员：
1. `client`: 表示与Azure Blob服务进行通信的客户端。
2. `retry_policy`: 用于在遇到错误时决定是否重试操作的策略。

通过这些结构体的实现，vector可以将采集到的日志数据通过AzureBlobSink<Svc>结构体提供的方法，将数据写入Azure Blob存储中的指定容器和Blob。同时，AzureBlobSvc结构体提供了与Azure Blob服务的交互能力，支持错误重试等功能。

