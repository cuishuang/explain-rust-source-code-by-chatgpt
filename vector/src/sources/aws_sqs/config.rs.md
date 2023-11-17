# File: vector/src/sources/aws_sqs/config.rs

在Rust生态中，vector是一个数据处理工具，用于收集、传输和转换日志和事件数据。vector的vector/src/sources/aws_sqs/config.rs文件是用于配置AWS Simple Queue Service (SQS)源的文件。

在该文件中，有几个struct，包括AwsSqsConfig、ClientConfig、InputConfig和SqsClientWrapper。下面详细介绍每个struct的作用：

1. AwsSqsConfig：此struct用于配置AWS SQS源的参数。它包含以下字段：
   - region：AWS SQS所在的区域。
   - queue_url：运行SQS的队列的URL。
   - consumer_sqs_optional: 指示是否使用AWS SQS作为vector的输入源。
   - client_config：一个ClientConfig结构，用于配置AWS SDK客户端。

2. ClientConfig：此struct用于配置AWS SDK客户端的参数。它包含以下字段：
   - access_key: AWS访问密钥。
   - secret_key: AWS秘密密钥。
   - session_token: AWS会话令牌，可选。
   - region: AWS区域。

3. InputConfig：此struct用于配置输入数据的参数。它包含以下字段：
   - encoding: 输入数据的编码方式。
   - healthcheck_interval_secs: 健康检查间隔时间（以秒为单位）。
   - request_timeout_secs: 请求超时时间（以秒为单位）。
   - connect_timeout_secs: 连接超时时间（以秒为单位）。

4. SqsClientWrapper：此struct是一个包装器，用于与AWS SQS服务进行交互。它实现了AWS SDK的Client trait，提供了SQS操作的具体实现。

通过配置这些struct的参数，可以定制化vector的AWS SQS源的行为，包括所在区域、队列URL、访问身份验证和其他与传输和处理数据相关的设置。

总而言之，vector/src/sources/aws_sqs/config.rs文件定义了用于配置和管理AWS SQS源的参数结构体，并提供了一个用于与AWS SQS服务交互的包装器。

