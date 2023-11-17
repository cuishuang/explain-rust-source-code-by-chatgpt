# File: vector/src/sources/aws_s3/sqs.rs

在Rust生态的vector项目中，vector/src/sources/aws_s3/sqs.rs文件的作用是定义与AWS Simple Queue Service（SQS）相关的功能。

该文件中定义了一些与SQS交互的数据结构、错误类型和事件类型。

下面是对这些结构的详细介绍：

1. Config：该结构体存储了SQS连接所需的配置信息，包括AWS的访问密钥、区域信息等。

2. State：该结构体表示SQS源的状态信息，如最后一个处理的消息的句柄、批处理的大小等。

3. Ingestor：该结构体表示SQS源的实例，它将从SQS队列中拉取消息并将其发送到数据处理流程中。

4. IngestorProcess：该结构体表示SQS消息的进程，包含了一个SQS消息和其他一些相关信息。

5. SnsNotification：该结构体表示AWS SNS（Simple Notification Service）的通知消息。

6. S3TestEvent：该结构体表示SQS测试事件，主要用于单元测试。

7. S3Event：该结构体表示AWS S3的事件。

8. S3EventRecord：该结构体表示AWS S3事件的记录，包含了一些关于S3事件发生的详细信息。

9. S3EventVersion：该枚举表示S3事件的版本。

10. S3EventName：该枚举表示S3事件的名称。

11. S3Message：该结构体表示AWS S3的消息，包含了一些与S3对象相关的信息。

12. S3Bucket：该结构体表示一个S3存储桶。

13. S3Object：该结构体表示S3中的一个对象。

下面是对这些错误类型和事件类型的详细介绍：

1. IngestorNewError：该枚举表示在创建Ingestor实例时可能出现的错误类型。

2. ProcessingError：该枚举表示在处理SQS消息时可能出现的错误类型。

3. SqsEvent：该枚举表示SQS事件的类型，可以为消息接收、消息删除等。

这些结构体和枚举类型的定义提供了与SQS交互的功能，并且可以将SQS队列中的消息传递到数据处理流程中。

