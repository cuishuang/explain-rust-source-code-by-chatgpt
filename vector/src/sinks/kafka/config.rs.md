# File: vector/src/sinks/kafka/config.rs

文件 config.rs 的作用是定义 KafkaSinkConfig 结构体和 KafkaRole 枚举，并为它们实现一些方法和转换函数。

KafkaSinkConfig 结构体是用来存储配置 Kafka Sink 的参数和选项的，它有以下字段：

- bootstrap_servers：Kafka 服务器的地址。
- security_protocol：安全协议，比如 "SSL"、"SASL_PLAINTEXT" 等。
- sasl_mechanism：SASL 机制，比如 "PLAIN"、"GSSAPI" 等。
- sasl_username：SASL 认证的用户名。
- sasl_password：SASL 认证的密码。
- kafka_topic：Kafka 主题名称。
- acks：消息确认模式，可以是 "0"、"1"、"all"。
- flush_timeout: 刷新超时时间，以毫秒为单位。
- max_buf_size: 最大缓冲区大小，以字节为单位。
- compression: 消息压缩方法，可以是 "none"、"gzip"、"snappy" 等。

KafkaRole 枚举是用来表示 Kafka 的角色，有以下几个成员：

- Producer：以生产者角色发送消息到 Kafka。
- Consumer：以消费者角色从 Kafka 接收消息。

KafkaSinkConfig 结构体和 KafkaRole 枚举的目的是通过配置参数和选项来定义如何连接到 Kafka 服务器、发送或接收消息，并提供了一些相关方法和转换函数，以方便使用和配置 Kafka Sink。

这些结构体和枚举的定义和功能对于 Rust 生态的 vector 项目来说非常重要，因为它们定义了与 Kafka 之间的连接和交互的配置信息和角色的抽象。通过配置参数和选项，可以根据实际需求灵活使用 Kafka 作为数据的流转工具。

