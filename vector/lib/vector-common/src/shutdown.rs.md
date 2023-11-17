# File: vector/lib/vector-common/src/shutdown.rs

在Rust生态vector项目中，`vector-common/src/shutdown.rs`文件的作用是实现了一个用于处理系统关闭信号的模块。

该模块中定义了以下几个关键结构体：

1. `ShutdownSignalToken`：这是一个可复制（Copy）的结构体，用于创建一个与系统关闭信号相关的令牌。当有其他线程或组件需要注册关闭信号时，可以通过复制这个令牌来确保注册者和实际信号之间的解耦。

2. `ShutdownSignal`：这是一个具有进一步功能的结构体，用于传播系统关闭信号。通过调用`shutdown()`方法，您可以向该信号发送关闭请求，并通过`wait()`方法等待关闭信号执行完毕。此外，还可以在创建`ShutdownSignal`实例时传入自定义的回调函数，在接收到关闭信号时执行。

3. `SourceShutdownCoordinator`：这是一个用于跟踪数据源的结构体，它管理了该源的关闭状态。每个数据源都可以使用该结构体进行注册，并在关闭信号发出时执行关闭逻辑。它内部包含一个原子布尔标志，用于表示源是否已关闭，并提供了`is_shutdown()`方法用于检查关闭状态。此外，还有一个内部的`ShutdownSignalToken`，可用于在注册数据源时分发关闭信号。

综上所述，`vector-common/src/shutdown.rs`文件提供了一个可重用的模块，用于处理系统关闭信号。它通过提供`ShutdownSignalToken`、`ShutdownSignal`和`SourceShutdownCoordinator`等结构体，为不同的组件实现了优雅的关闭机制，并帮助用户完成资源的释放和清理工作。

