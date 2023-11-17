# File: vector/src/sources/aws_kinesis_firehose/handlers.rs

在Rust生态vector项目的源代码中，vector/src/sources/aws_kinesis_firehose/handlers.rs文件的作用是实现处理来自AWS Kinesis Firehose的数据的处理器。该文件中定义了一些与处理Firehose数据流相关的结构体和枚举。

在handlers.rs文件中，有以下几个重要的结构体：

1. `FirehoseHandler`：实现了处理来自Firehose数据的处理器的主要结构体。它负责接收来自Firehose数据流的记录，并通过`process_record`方法将记录发送给下游处理链。

2. `Buffer`：用于缓冲记录的结构体。它通过`store`方法接收记录，并在缓存达到一定大小时将缓存的记录发送给下游处理链。

3. `Context`：上下文结构体，保存有关处理过程中的一些状态和属性。它包含了与Firehose数据流相关的信息，如数据流名称和数据流的处理位置等。

而在handlers.rs文件中，还定义了以下几个枚举：

1. `RecordDecodeError`：记录解码错误的枚举。它包含了多个错误类型，如无效的记录格式、解码时遇到非法字符等。

这些结构体和枚举在整个Firehose数据处理过程中起到了关键的作用。`FirehoseHandler`负责处理数据流和缓冲记录，`Buffer`负责记录的缓冲管理，`Context`保存有关数据处理的上下文信息，并通过`RecordDecodeError`枚举来处理解码错误。通过这些结构体和枚举，可以实现对Firehose数据流的有效处理和解码。

