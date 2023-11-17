# File: vector/src/internal_events/docker_logs.rs

`docker_logs.rs`是Rust生态`vector`项目中的一个文件，用于处理Docker日志相关的事件。

以下是各个结构体的详细作用：

1. `DockerLogsEventsReceived<'a>`：表示接收到Docker日志事件的结构体。它包含了接收到的日志事件的元数据和内容。

2. `DockerLogsContainerEventReceived<'a>`：表示接收到Docker容器事件的结构体。它包含了接收到的容器事件的元数据和内容。

3. `DockerLogsContainerWatch<'a>`：表示监视Docker容器日志的结构体。它包含了要监视的容器的元数据和相关信息。

4. `DockerLogsContainerUnwatch<'a>`：表示取消监视Docker容器日志的结构体。它包含了要取消监视的容器的元数据和相关信息。

5. `DockerLogsCommunicationError<'a>`：表示与Docker日志通信发生错误的结构体。它包含了错误的元数据和相关信息。

6. `DockerLogsContainerMetadataFetchError<'a>`：表示获取Docker容器元数据发生错误的结构体。它包含了错误的元数据和相关信息。

7. `DockerLogsTimestampParseError<'a>`：表示解析Docker日志时间戳发生错误的结构体。它包含了错误的元数据和相关信息。

8. `DockerLogsLoggingDriverUnsupportedError<'a>`：表示不支持的Docker日志驱动程序发生错误的结构体。它包含了错误的元数据和相关信息。

这些结构体被用于处理相应类型的Docker日志事件，以便在处理过程中进行相关的操作和错误处理。

