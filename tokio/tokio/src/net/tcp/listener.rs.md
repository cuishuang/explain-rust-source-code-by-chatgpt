# File: tokio/tokio/src/net/tcp/listener.rs

在tokio的源代码中，`listener.rs`文件位于`tokio/tokio/src/net/tcp`目录下，该文件的作用是实现TCP监听器相关的功能。

`TcpListener`结构体是一个封装了TCP监听器的类型。它负责创建和管理TCP监听套接字，并提供了一些方法来接受传入的TCP连接请求。

`TcpListener`结构体有三个重要的字段：

1. `listener`：一个内部的`std::net::TcpListener`实例，负责监听指定的IP地址和端口。

2. `io`：`io::PollEvented`类型的实例，负责监听套接字上的IO事件。

3. `accepts`：一个包含了`futures::stream::Fuse`类型的FIFO缓冲区，用于管理已经接受的连接句柄。

`TcpListener`结构体实现了`futures::stream::Stream` trait，所以可以作为一个异步流使用。可以使用`next()`方法来获取下一个传入的连接，也可以使用`try_next()`方法来尝试获取下一个传入的连接（不会阻塞）。

`TcpListener`结构体还提供了一些其他的方法，包括：

- `bind()`：绑定指定的IP地址和端口，创建一个`TcpListener`实例。
- `local_addr()`：返回`TcpListener`绑定的本地地址。
- `incoming()`：返回一个迭代器，用于获取所有传入的TCP连接。
- `poll_accept()`：等待并接受传入的TCP连接请求，并返回一个包含连接句柄的`Poll`结果。

