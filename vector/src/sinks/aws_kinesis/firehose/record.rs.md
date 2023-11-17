# File: vector/src/sinks/aws_kinesis/firehose/record.rs

在Rust生态vector项目的源代码中，`vector/src/sinks/aws_kinesis/firehose/record.rs`文件的作用是实现与AWS Kinesis Firehose服务进行通信和处理数据记录的功能。

具体来说，该文件定义了两个重要的结构体：`KinesisFirehoseRecord`和`KinesisFirehoseClient`。

`KinesisFirehoseRecord`结构体用于表示Kinesis Firehose服务接收到的数据记录。该结构体包含以下字段：
- `data`：记录的实际数据，以字节数组的形式表示。
- `delivery_stream_name`：数据记录要发送到的Kinesis Firehose Delivery Stream的名称。
- `partition_key`：用于分区定位和路由记录的分区键。

`KinesisFirehoseClient`结构体则用于实现与AWS Kinesis Firehose服务进行通信的客户端功能。该结构体包含以下方法：
- `send_record`：将数据记录发送到Kinesis Firehose服务的指定Delivery Stream中。
- `retry_on_failure`：当发送数据记录时遇到失败情况时，进行重试操作。

`KinesisFirehoseRecord`和`KinesisFirehoseClient`结构体是在AWS Kinesis Firehose数据源中使用的重要组件。数据流通常通过Kinesis Firehose Record API发送到Kinesis Firehose服务。`KinesisFirehoseRecord`结构体用于封装数据记录，并包含必要的元数据，以便由`KinesisFirehoseClient`发送到正确的Delivery Stream。`KinesisFirehoseClient`结构体负责与AWS Kinesis Firehose服务建立连接并处理与数据记录的交互。

这些结构体的实现使得Vector能够将数据发送到AWS Kinesis Firehose服务，从而实现了对该服务的集成和利用。

