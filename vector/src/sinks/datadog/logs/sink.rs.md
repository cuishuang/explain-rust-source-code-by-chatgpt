# File: vector/src/sinks/datadog/logs/sink.rs

文件`vector/src/sinks/datadog/logs/sink.rs`是`vector`项目中与DataDog日志相关功能的实现文件。它实现了一个用于将日志数据发送到DataDog的日志Sink。

下面分别介绍各个结构体和枚举的作用：

1. `EventPartitioner`：一个结构体，用于将事件分区到不同的DataDog日志批次。它根据事件的时间戳（如果有）来决定将事件发送到哪个批次。

2. `LogSinkBuilder<S>`：一个结构体，用于构建DataDog日志Sink。它封装了创建和配置DataDog日志Sink所需的参数，包括DataDog API密钥、批次大小、写入超时等。

3. `LogSink<S>`：一个结构体，代表DataDog日志Sink。它负责将日志数据发送到DataDog平台。它使用`EventPartitioner`将日志事件分区到不同的批次，并使用`JsonEncoding`将日志事件编码为JSON格式。

4. `JsonEncoding`：一个结构体，用于将日志事件编码为JSON格式。它将日志事件的字段转换为JSON对象，并将其序列化为JSON字符串。

5. `LogRequestBuilder`：一个结构体，用于构建发送给DataDog的日志请求。它封装了将要发送的日志事件数据，包括日志消息、标签、服务名称等。

以下是枚举的作用：

1. `RequestBuildError`：一个枚举，表示构建DataDog日志请求时可能发生的错误。它包括以下错误类型：
   - `RequestTooLarge`：请求超出了DataDog的最大允许大小限制。
   - `InvalidUtf8`：日志消息中包含非法的UTF-8字符。
   - `MessageTooLarge`：日志消息超出了DataDog的最大允许大小限制。

以上是对每个结构体和枚举的简要介绍。详细了解每个结构体和枚举的具体实现和作用，可以查看源代码文件`vector/src/sinks/datadog/logs/sink.rs`。

