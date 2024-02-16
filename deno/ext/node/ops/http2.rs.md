# File: /Users/fliter/rust-contribute/deno/ext/node/ops/http2.rs

在Deno项目的源代码中，`/Users/fliter/rust-contribute/deno/ext/node/ops/http2.rs`文件是用来实现HTTP2协议相关操作的。

该文件中包含以下几个结构体：

1. `Http2Client`：代表一个HTTP2客户端，用于发送HTTP2请求到服务器并接收响应。
2. `Http2ClientConn`：代表HTTP2客户端连接，维护与服务器的HTTP2连接。
3. `Http2ClientStream`：代表一个HTTP2客户端流，在HTTP2连接上发送和接收数据。
4. `Http2ClientResponseBody`：代表HTTP2客户端响应体，用于接收来自服务器的响应数据。
5. `Http2ServerConnection`：代表HTTP2服务器连接，维护与客户端的HTTP2连接。
6. `Http2ServerSendResponse`：用于向HTTP2客户端发送响应。
7. `Http2ClientResponse`：代表HTTP2客户端响应，包含响应头和响应体。

此外，还有以下几个枚举类型：

1. `DataOrTrailers`：表示HTTP2流中的数据或者标头。
2. `Http2StreamState`：表示HTTP2流的状态，如空闲、打开、半关闭等。

这些结构体和枚举类型的作用是为了在Deno项目中提供对HTTP2协议的支持，实现HTTP2客户端和服务器的功能。它们分别封装了HTTP2协议的各个组成部分，使得Deno可以通过这些结构体和枚举类型来发送和处理HTTP2请求和响应。

