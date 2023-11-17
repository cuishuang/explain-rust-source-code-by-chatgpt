# File: vector/src/sinks/azure_blob/request_builder.rs

在Rust生态的vector项目中，vector/src/sinks/azure_blob/request_builder.rs文件的作用是构建用于与Azure Blob Storage进行通信的请求。

该文件中定义了一个名为AzureBlobRequestBuilder的结构体，该结构体负责构建RESTful的HTTP请求，以便与Azure Blob Storage进行交互。这个构建器依赖于AzureBlobRequestOptions结构体来配置请求的各种选项。

AzureBlobRequestOptions是一个枚举类型，它包含了几个不同的结构体，分别用于配置不同种类的请求选项。以下是这些结构体及其作用的详细介绍：

1. LeaseBlobOptions: 用于配置与租约相关的操作，如创建、续租、释放、检查租约等。
2. ListBlobsOptions: 用于配置列举Blob的操作，如指定前缀、分隔符等。
3. PutBlockBlobOptions: 用于配置上传块Blob的操作，如指定内容类型、内容编码、内容语言、内容MD5哈希等。
4. GetBlobOptions: 用于配置获取单个Blob的操作，如指定快照、范围、版本等。
5. PutAppendBlobOptions: 用于配置追加Blob的操作，如指定内容类型、内容编码等。
6. PutPageBlobOptions: 用于配置分页Blob的操作，如指定内容类型、内容编码、MD5哈希等。
7. PutOrUpdateBlobMetadataOptions: 用于配置Blob元数据的操作，如指定元数据键值对等。
8. CopyBlobOptions: 用于配置复制Blob的操作，如指定标头、源URL等。
9. SetBlobTierOptions: 用于配置设置Blob层级的操作，如指定目标层级等。

这些结构体支持不同类型的请求选项，通过填充这些选项，可以灵活地配置与Azure Blob Storage的交互，实现各种不同的操作需求。

