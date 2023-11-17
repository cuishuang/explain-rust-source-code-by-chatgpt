# File: vector/src/sources/aws_kinesis_firehose/models.rs

在Rust生态vector项目中，vector/src/sources/aws_kinesis_firehose/models.rs文件起到了定义AWS Kinesis Firehose相关模型的作用。

1. FirehoseRequest结构体：表示发送到AWS Kinesis Firehose的请求。该结构体包含以下字段：
   - delivery_stream_name: 发送到的Kinesis Firehose交付流的名称。
   - records: 发送的记录列表，记录是EncodedFirehoseRecord结构体的向量。

2. EncodedFirehoseRecord结构体：表示AWS Kinesis Firehose接收的记录的编码版本。该结构体包含以下字段：
   - data: 记录的数据内容，以字节数组的形式存储。
   - partition_key: 记录的分区键，用于确定记录将存储在Kinesis Firehose中的哪个分区。

3. FirehoseResponse结构体：表示从AWS Kinesis Firehose接收的响应。该结构体包含以下字段：
   - record_id: 每个成功记录的唯一标识符。
   - error_message: 如果记录处理失败，则包含与失败相关的错误消息。

这些结构体定义了与AWS Kinesis Firehose交互时使用的数据模型。FirehoseRequest结构体用于构建发送到Kinesis Firehose的请求，EncodedFirehoseRecord结构体用于表示发送的记录，FirehoseResponse结构体用于表示从Kinesis Firehose接收的响应。这些结构体使得在Vector中与Kinesis Firehose进行通信的过程中能够更方便地处理和传递数据。

