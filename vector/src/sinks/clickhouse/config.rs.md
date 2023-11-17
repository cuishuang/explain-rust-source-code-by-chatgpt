# File: vector/src/sinks/clickhouse/config.rs

在Rust生态vector项目的源代码中，vector/src/sinks/clickhouse/config.rs文件的作用是定义了与ClickHouse数据存储系统相关的配置选项和结构体。

该文件主要包含以下几个结构体：

1. ClickhouseConfig: 这是一个顶层的配置结构体，用于存储与ClickHouse相关的所有配置选项。它包含以下字段：
   - `host`: ClickHouse服务器的主机名或IP地址。
   - `port`: ClickHouse服务器的端口号。
   - `database`: 数据库名称，用于指定在ClickHouse中存储数据的数据库。
   - `table`: 表名称，用于指定在ClickHouse中存储数据的表。
   - `username`: 连接ClickHouse服务器所需的用户名。
   - `password`: 连接ClickHouse服务器所需的密码。
   - `compression`: 是否启用数据压缩选项。

2. EncodingConfig: 这是一个编码配置结构体，用于定义在将数据写入ClickHouse之前如何对数据进行编码。它包含以下字段：
   - `format`: 指定数据的格式，如JSON、CSV等。
   - `delimiter`: 如果选择CSV格式，可以指定列分隔符。
   
3. BufferConfig: 这是一个缓冲配置结构体，用于定义在将数据写入ClickHouse之前如何缓冲数据。它包含以下字段：
   - `type`: 缓冲类型，可以是 "memory" 或 "disk"。
   - `size`: 缓冲区大小，用于控制在将数据写入ClickHouse之前可以缓冲的数据量。
   - `when_full`: 当缓冲区满时，可以选择采取的操作，如等待、丢弃等。

这些配置结构体将用于在Vector的ClickHouse sink中指定所需的配置选项，以便将数据发送至ClickHouse数据库。通过这些结构体，用户可以自定义ClickHouse的主机和端口、连接凭据、数据编码格式、缓冲方式等，以满足特定的需求。

在代码中，这些配置结构体的目的是提供一个规范的方式来定义和传递与ClickHouse相关的配置选项，以方便用户在Vector中配置并使用ClickHouse sink的功能。配置选项的值将在Vector的运行时中解析，并用于建立与ClickHouse的连接，并控制数据的编码和缓冲方式。

