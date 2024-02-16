# File: /Users/fliter/rust-contribute/deno/ext/http/request_body.rs

在Deno项目的源代码中，/Users/fliter/rust-contribute/deno/ext/http/request_body.rs文件的作用是处理HTTP请求的请求体。

文件中包含了两个结构体：ReadFuture(Incoming)和HttpRequestBody(AsyncRefCell<Peekable<ReadFuture>>。

1. ReadFuture(Incoming):
   - 这个结构体是Future类型，表示将读取请求体的异步操作封装为Future。
   - Incoming参数用于接收网络连接，并在连接上接收请求。

2. HttpRequestBody(AsyncRefCell<Peekable<ReadFuture>>):
   - 这个结构体表示HTTP请求的请求体，并通过AsyncRefCell包装ReadFuture，使其具备异步读取能力。
   - Peekable用于在不消耗数据的情况下查看读取流中的下一个元素。
   - HttpRequestBody的主要作用是异步读取请求体的内容，并根据需要提供相应的方法来处理请求体，例如读取JSON数据或表单数据。

这些结构体是Deno项目中处理HTTP请求的一部分，负责处理请求体的读取和处理。ReadFuture用于读取请求体的Future操作，而HttpRequestBody则是对请求体的封装，提供了相关方法来处理请求体数据。

