# File: vector/src/internal_events/common.rs

在Rust生态vector项目的源代码中，vector/src/internal_events/common.rs文件起到了定义一系列内部事件的作用。这些内部事件用于表示在Vector数据流处理过程中的不同情况和状态。

下面是对每个结构体的详细介绍：

1. EndpointBytesReceived<'a>：表示某个端点（endpoint）接收到的字节数。这个结构体包含了有关接收字节数的信息，如端点名称和字节数。

2. EndpointBytesSent<'a>：表示某个端点发送的字节数。类似于EndpointBytesReceived，这个结构体包含了有关发送字节数的信息。

3. SocketOutgoingConnectionError<E>：表示建立套接字的传出连接时发生的错误。E是用于描述错误类型的泛型参数。

4. StreamClosedError：表示流已关闭的错误。当流无法继续读取或写入时，会触发此错误。

5. RequestCompleted：表示请求已完成。当一个请求成功处理完毕时，会发送这个事件。

6. CollectionCompleted：表示收集已完成。当收集数据完成时，会发送这个事件。

7. SinkRequestBuildError<E>：表示构建Sink请求时出错。E是用于描述错误类型的泛型参数。

这些内部事件结构体是为了帮助向Vector内部的事件流中发送特定的事件，以便在适当的时候进行处理和响应。这些事件可用于监控和管理Vector的运行状态，以及进行错误处理和其他操作。通过这些事件结构体，可以更加灵活地控制和管理Vector的行为。

