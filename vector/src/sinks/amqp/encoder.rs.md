# File: vector/src/sinks/amqp/encoder.rs

在Rust生态vector项目中，`vector/src/sinks/amqp/encoder.rs`文件的作用是实现将消息编码为AMQP（Advanced Message Queuing Protocol）协议格式的函数和类型。

该文件中的`AmqpEncoder`结构体是消息编码器的核心，它包含了将消息封装为AMQP帧的方法，以及编码帧的方法。这个结构体还包含一个内部的`InnerEncoder`结构体，用来处理AMQP帧的编码逻辑。

具体来说，`AmqpEncoder`结构体用于将从vector接收到的消息转换为AMQP帧。它提供了以下方法：

1. `new`: 根据给定的AMQP配置创建一个新的`AmqpEncoder`实例。
2. `set_content_type`: 设置 AMQP 帧的内容类型。
3. `encode_batch`: 将批量消息编码为 AMQP 帧。

`AmqpEncoder`结构体内部使用`InnerEncoder`结构体来执行 AMQP 编码的细节。该结构体的目的是封装 AMQP 协议的低级细节，并为上层提供一个简单的 API。

`InnerEncoder`结构体提供了以下方法：

1. `encode_properties`: 编码 AMQP 帧的属性。
2. `encode_headers`: 编码 AMQP 帧的头部。
3. `encode_body`: 编码 AMQP 帧的消息体。

在整个过程中，`AmqpEncoder`和`InnerEncoder`结构体通过使用 AMQP 库来将消息转换为 AMQP 帧。这些结构体的作用是将 vector 接收到的消息转换为符合 AMQP 协议的格式，并将其发送到 AMQP 目标。

