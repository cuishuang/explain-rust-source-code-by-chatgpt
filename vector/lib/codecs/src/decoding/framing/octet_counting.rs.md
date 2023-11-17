# File: vector/lib/codecs/src/decoding/framing/octet_counting.rs

在Rust生态vector项目的源代码中，`vector/lib/codecs/src/decoding/framing/octet_counting.rs`文件的作用是实现了一个用于解码帧的 Octet Counting 编码器。

OctetCountingDecoderConfig结构体用于配置 Octet Counting 编码器。它包含以下字段：
- `max_frame_size`: 表示最大帧大小，即解码的数据帧的最大长度。如果数据帧的长度超过了该值，则会触发一个错误。

OctetCountingDecoderOptions结构体用于配置 Octet Counting 编码器的选项。它包含以下字段：
- `config`: 一个可选的OctetCountingDecoderConfig结构体，用于设置Octet Counting编码器的配置。如果不提供任何配置，编码器将使用默认配置。

OctetCountingDecoder结构体是实现了数据解码逻辑的 Octet Counting 编码器。它包含以下字段：
- `state`: 表示解码器当前的状态，类型为State枚举的一个变量。
- `frame_size`: 表示当前解码的帧的大小。如果为None，则表示还未确定帧的大小。
- `buffer`: 一个可变的字节缓冲区，用于存储从输入中读取的字节。
- `frame_buffer`: 一个可变的字节缓冲区，用于存储当前解码的帧的数据。

State枚举类型包含解码器的状态。它有以下几个变体：
- `WaitingForSize`: 表示解码器正在等待帧的大小信息。
- `WaitingForData`: 表示解码器正在等待帧的数据。
- `Finished`: 表示解码器已完成帧的解码。

简单来说，该文件中的代码实现了一个 Octet Counting 解码器，用于解码以字节计数方式编码的数据帧。解码器根据指定的配置和选项，解析输入的字节流，从中提取出帧的大小和数据。同时，它会处理边界情况，比如当帧大小超过最大帧大小配置时，会触发错误。

