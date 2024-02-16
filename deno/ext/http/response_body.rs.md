# File: /Users/fliter/rust-contribute/deno/ext/http/response_body.rs

在Deno项目的源代码中，/Users/fliter/rust-contribute/deno/ext/http/response_body.rs文件的作用是定义了HTTP响应的主体部分的处理方式。

- ResourceBodyAdapter struct 是一个适配器，它实现了 tokio::io::AsyncRead 和 tokio::io::AsyncWrite trait，用于将请求的主体数据转换成响应主体数据的流。
- GZipResponseStream struct 是一个gzip压缩的响应流，它实现了 tokio::io::AsyncRead 和 tokio::io::AsyncWrite trait，用于将gzip压缩的数据写入到响应。
- BrotliEncoderStateWrapper struct 是对brotli压缩器状态的封装，用于进行brotli压缩。
- BrotliResponseStream struct 是一个brotli压缩的响应流，它实现了 tokio::io::AsyncRead 和 tokio::io::AsyncWrite trait，用于将brotli压缩的数据写入到响应。

PollFrame trait 是一个trait别名，它定义了处理流数据的帧的函数。

- ResponseStreamResult enum 是一个枚举类型，它定义了不同的响应流结果，如成功、被中断等。
- Compression enum 是一个枚举类型，它定义了不同的压缩方式，如不压缩、gzip、brotli等。
- ResponseStream struct 是一个响应流的结构体，它实现了 tokio::io::AsyncWrite trait，用于将响应数据写入到连接。
- ResponseBytesInner struct 是响应数据的表示结构体，包含了响应头和主体数据等信息。
- GZipState struct 是gzip状态的枚举类型，表示gzip压缩的处理状态。
- BrotliState struct 是brotli状态的枚举类型，表示brotli压缩的处理状态。

这些结构体、trait和枚举类型共同实现了在HTTP请求中处理响应主体的功能，包括对压缩和流传输的支持。

