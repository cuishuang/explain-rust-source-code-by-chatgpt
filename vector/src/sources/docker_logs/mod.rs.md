# File: vector/src/sources/docker_logs/mod.rs

在Rust生态的vector项目中，`vector/src/sources/docker_logs/mod.rs`文件是用于实现Docker日志源的模块。该模块提供了与Docker日志相关的配置、核心源、源、事件流构建器和容器信息等功能。

- `DockerLogsConfig`是用于配置Docker日志源的结构体。它定义了一些参数，如Docker Daemon的地址、容器过滤条件等。

- `DockerLogsSourceCore`是Docker日志源的核心结构体。它维护了与Docker Daemon通信所需的一些状态信息，如Docker API客户端、事件流构建器等。

- `DockerLogsSource`是Docker日志源的主要结构体。它基于`DockerLogsSourceCore`实现了`Source` trait，可以通过调用`generate`方法生成日志事件流。

- `EventStreamBuilder`是一个用于构建事件流的结构体。它提供了一些方法来设置事件流的处理器和过滤条件，并最终生成一个用于返回的`EventStream`对象。

- `ContainerId`是一个表示容器ID的字节序列。

- `ContainerState`是一个枚举，表示容器的状态，如运行中、退出、等待等。

- `ContainerLogInfo`是一个结构体，保存了容器的相关日志信息，如日志路径、标签等。

- `ContainerMetadata`是一个结构体，保存了容器的元数据，如容器ID、名称等。

- `ErrorPersistence`是一个枚举，表示错误的持久性类型，可用于处理错误时的决策，如忽略、丢弃、中止等。这些枚举项允许根据错误的类型来确定相应的处理方式。

