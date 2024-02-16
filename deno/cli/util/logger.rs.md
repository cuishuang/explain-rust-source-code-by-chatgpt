# File: /Users/fliter/rust-contribute/deno/cli/util/logger.rs

在Deno项目的源代码中，/Users/fliter/rust-contribute/deno/cli/util/logger.rs文件的作用是定义了Deno的日志记录器功能。这个文件包含了用于在Deno命令行界面显示各种类型的日志消息的实现。

在这个文件中，有几个结构体扮演着不同的角色：

1. CliLogger 结构体是用于创建Deno的日志记录器的工具，它接受一个参数 env_logger::Logger，这是一个第三方日志记录库。CliLogger 实现了 Deref 和 DerefMut traits，以便可以将其作为一种类型的指针来使用。这样做的目的是让 CliLogger 作为一个“重度引用”存在，以便日志系统可以在程序的不同部分被共享和修改。

2. LoggerWriter 结构体是 CliLogger 的一个包装器，它允许将各种类型的日志消息写入到日志系统。LoggerWriter 实现了 Write trait，这样就可以使用标准库中的 write! 和 writeln! 宏来向日志系统写入数据。LoggerWriter 还使用了一个 AtomicBool 类型的标志位来表示是否启用日志写入操作。

这些结构体的主要作用是提供了一个灵活的日志记录机制，可以在运行时灵活地控制日志的启用和禁用，并且可以将各种级别的日志消息输出到不同的位置（如控制台、文件等）。通过使用 env_logger::Logger 库，Deno可以在不同的运行环境中提供适当的日志记录功能。

总结来说，/Users/fliter/rust-contribute/deno/cli/util/logger.rs 文件的作用是定义了Deno项目的日志记录器功能，通过 CliLogger 和 LoggerWriter 结构体，它提供了一种有效的方式来管理和输出各种类型的日志消息。

