# File: vector/src/sinks/util/tcp.rs

`tcp.rs`文件是Rust生态vector项目中vector库的一个源代码文件，它定义了与TCP相关的一些结构和错误类型，用于支持将数据发送到TCP目标。

- `TcpSinkConfig`是一个结构体，用于配置TCP数据发送器（`TcpSink`）的参数。它包含了目标TCP服务器的IP地址、端口号以及一些可选的TLS配置参数。

- `TcpConnector`是一个辅助结构体，用于建立与目标TCP服务器的连接。它处理了TCP连接的建立和关闭过程。

- `TcpSink<E>`是实际的TCP数据发送器，它实现了Vector的`Sink` trait，使其可以将数据发送到TCP服务器。它通过使用TCP连接从内部缓冲区异步地将数据写入TCP目标。

- `TcpError`是一个枚举类型，定义了与TCP相关的错误类型。它包括了与TCP连接建立、发送和关闭相关的各种错误场景。这些错误包括无法解析目标地址、连接超时、传输错误等。

总的来说，`tcp.rs`文件中的这些结构和枚举类型提供了TCP数据发送所需的配置和功能，使得Vector能够支持将数据实时地发送到TCP目标。

