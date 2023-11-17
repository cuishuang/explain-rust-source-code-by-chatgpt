# File: vector/src/sinks/pulsar/request_builder.rs

request_builder.rs文件是vector项目中用于构建Pulsar请求的模块之一。它包含了两个重要的结构体：PulsarMetadata和PulsarRequestBuilder。

PulsarMetadata结构体用于包装Pulsar相关的元数据，例如topic名称、生产者ID等。这些元数据用于构建Pulsar请求时提供必要的上下文信息。

PulsarRequestBuilder结构体是用于构建Pulsar请求的建造者模式。它提供了一系列的方法来设置和配置Pulsar请求的不同属性，以便发送到Pulsar主题或队列中。以下是PulsarRequestBuilder结构体中的一些重要方法：

- `topic(topic: impl Into<String>) -> Self`: 设置Pulsar请求的主题名称。
- `producer_name(name: impl Into<String>) -> Self`: 设置Pulsar请求的生产者名称。
- `producer_id(id: impl Into<String>) -> Self`: 设置Pulsar请求的生产者ID。
- `sequence_id(id: impl Into<i64>) -> Self`: 设置Pulsar请求的序列ID，用于有序写入消息。
- `payload(payload: impl Into<Vec<u8>>) -> Self`: 设置Pulsar请求的消息体。

这些方法可以通过链式调用来设置Pulsar请求的不同属性。最终，PulsarRequestBuilder结构体可以使用`build`方法生成最终的Pulsar请求对象。

这个文件的作用是提供一个方便的接口，使开发者可以使用PulsarRequestBuilder来构建Pulsar请求，并通过PulsarMetadata将请求与相应的元数据关联起来。这样，开发者可以更容易地发送和处理与Pulsar相关的消息。

