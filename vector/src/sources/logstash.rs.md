# File: vector/src/sources/logstash.rs

在Rust生态中的vector项目中，`vector/src/sources/logstash.rs`这个文件的作用是实现了与Logstash的交互。具体来说，该文件提供了与Logstash进行连接和通信的功能。

以下是对于每个struct的详细介绍：

1. `LogstashConfig`: 维护了与Logstash连接的配置信息，例如主机地址、端口号等。

2. `LogstashSource`: 负责读取Logstash服务器发送的数据，并将其放入内部缓冲区。

3. `LogstashAcker`: 实现了与Logstash的数据确认交互逻辑，用于确认已经成功接收并处理了Logstash发送的数据。

4. `LogstashDecoder`: 解码从Logstash接收到的数据，并将其转化为可供后续处理的格式。具体的解码逻辑根据数据的类型和协议的版本不同而有所不同。

5. `LogstashEventFrame`: 表示从Logstash接收到的一个事件的数据结构，包括事件的元数据和具体的数据内容。

以下是对于每个enum的详细介绍：

1. `LogstashDecoderReadState`: 表示解码器的读取状态，包括等待数据、读取数据、读取数据失败等。

2. `DecodeError`: 表示解码过程中可能发生的错误类型，例如数据格式错误、解析错误等。

3. `LogstashProtocolVersion`: 标识Logstash的协议版本。

4. `LogstashFrameType`: 表示Logstash发送的帧的类型，例如事件帧、确认帧等。

总的来说，`vector/src/sources/logstash.rs`文件中的struct和enum实现了与Logstash进行连接、通信和数据解析的功能，为vector项目提供了与Logstash的集成能力。

