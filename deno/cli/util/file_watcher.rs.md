# File: /Users/fliter/rust-contribute/deno/cli/util/file_watcher.rs

在Deno项目的源代码中，/Users/fliter/rust-contribute/deno/cli/util/file_watcher.rs文件的作用是实现文件变化监视器，用于监听文件系统中文件或目录的变化。

以下是对文件中几个重要结构的介绍：

1. DebouncedReceiver：这是一个结构体，用于接收文件变化事件。它是一个 mpsc（multiple-producer, single-consumer）通道，可以从中读取文件变化事件。

2. PrintConfig：这是一个结构体，用于记录打印配置信息。它包含了打印颜色和是否打印文件变化事件等。

3. WatcherCommunicator：这是一个结构体，用于与文件变化监视器进行通信。它包含了与系统文件变化事件处理相关的方法和属性。

WatcherRestartMode是一个枚举类型，表示文件变化监视器的重新启动模式。它包含以下几个枚举值：

1. Restart: 文件变化监视器会在收到文件变化事件后重新启动。

2. Ignore: 文件变化监视器会忽略文件变化事件。

3. Shutdown: 文件变化监视器会关闭。

文件变化监视器在Deno项目中的作用是监听文件系统中的文件或目录的变化，并根据变化进行相应操作，例如重新编译文件，重新加载模块等。这可以帮助开发人员在开发过程中实时捕捉文件变化，提高开发效率。

