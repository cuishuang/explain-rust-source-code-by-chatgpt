# File: vector/src/sinks/azure_blob/config.rs

在Rust生态向量（vector）项目中，vector/src/sinks/azure_blob/config.rs文件的作用是定义了与Azure Blob存储相关的配置。它定义了一个名为AzureBlobSinkConfig的结构体，该结构体是用于存储与Azure Blob存储相关的配置项的。

AzureBlobSinkConfig结构体具有以下作用：

1. `conn_string`: 这是Azure Blob存储的连接字符串，用于连接到Azure Blob存储帐户。连接字符串包含账户名称、账户密钥以及Blob存储终结点的相关信息。

2. `container`: 这是Azure Blob存储中的容器名称。容器类似于文件夹，用于组织和管理存储的Blob对象。应指定要将事件数据写入的特定容器。

3. `blob_name_template`: 这是用于生成Blob对象名称的模板。模板可以包含动态字段，如时间戳或事件相关的元数据。根据模板生成的Blob对象名称将用于存储写入Azure Blob存储的事件数据。

4. `batch_size`: 这是用于配置批量写入操作时的事件数据数量。当达到批量大小时，Vector将触发将事件数据写入Azure Blob存储的操作。这可以提高写入的效率。

5. `max_connection_attempts`: 这是在连接到Azure Blob存储时的最大连接尝试次数。如果连接失败，Vector将尝试重新连接，直到达到最大尝试次数为止。

通过配置AzureBlobSinkConfig结构体的各项属性，可以对连接到Azure Blob存储、容器名称、Blob对象名称模板以及批量写入等操作进行详细的配置和定制。这使得在Vector中集成Azure Blob存储变得更加灵活和可配置。

