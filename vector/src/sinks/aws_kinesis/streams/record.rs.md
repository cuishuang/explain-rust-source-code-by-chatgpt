# File: vector/src/sinks/aws_kinesis/streams/record.rs

在Rust生态的vector项目中，vector/src/sinks/aws_kinesis/streams/record.rs文件的作用是提供与AWS Kinesis流数据记录相关的功能。

该文件中定义了两个结构体：KinesisStreamRecord和KinesisStreamClient。

1. KinesisStreamRecord结构体：
   KinesisStreamRecord结构体表示一个AWS Kinesis数据记录。每个记录可以包含不同的数据，例如事件消息、日志等。该结构体具有以下字段：

   - shard_id：记录所属的Kinesis分片ID。
   - sequence_number：记录的序列号，用于唯一标识记录在分片中的位置。
   - data：记录的实际数据，以字节数组形式存储。

   KinesisStreamRecord结构体还实现了相应的方法，用于序列化和反序列化记录数据。

2. KinesisStreamClient结构体：
   KinesisStreamClient结构体是与AWS Kinesis流进行交互的客户端。它利用AWS SDK提供的功能，可以发送数据记录到Kinesis流或从中读取数据记录。具体来说，该结构体具有以下功能：

   - `create_stream_record`方法：用于发送数据记录到Kinesis流。
   - `read_stream_records`方法：用于读取Kinesis流中的数据记录。
   - `delete_stream_record`方法：用于从Kinesis流中删除指定的数据记录。

   此外，KinesisStreamClient结构体还包含了与AWS Kinesis流的连接和身份验证相关的配置参数，例如AWS区域、访问密钥等。

总之，vector/src/sinks/aws_kinesis/streams/record.rs文件提供了处理AWS Kinesis流数据记录的功能，并定义了相应的结构体KinesisStreamRecord和KinesisStreamClient来支持该功能。

