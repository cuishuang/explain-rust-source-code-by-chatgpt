# File: tokio/tokio-util/src/codec/any_delimiter_codec.rs

在tokio-util库的`any_delimiter_codec.rs`文件中，定义了一个名为`AnyDelimiterCodec`的结构体和一个名为`AnyDelimiterCodecError`的枚举。

`AnyDelimiterCodec`结构体是一个编解码器，用于处理数据流的分隔符。它的作用是将输入的数据流分割为一系列的帧（frame），每个帧以指定的分隔符结尾。这样可以方便地处理基于分隔符的协议。

`AnyDelimiterCodec`结构体有两个类型参数：`T`和`D`。其中，`T`是要被编解码器处理的帧的类型，而`D`是分隔符的类型。

`AnyDelimiterCodec`结构体实现了tokio库中的`Decoder`和`Encoder` trait。`Decoder` trait负责将输入的字节流解码为帧，而`Encoder` trait负责将帧编码为字节流。

`AnyDelimiterCodec`结构体具有以下方法：
- `new`：创建一个新的`AnyDelimiterCodec`实例。
- `delimiter`：设置分隔符。
- `complete_frame`：将当前的缓冲数据视为一个完整的帧，并将其加入到输出列表中。
- `decode`：从输入字节流中解码帧，返回解码的结果。

`AnyDelimiterCodecError`枚举定义了`AnyDelimiterCodec`结构体可能出现的错误。它包含以下几个变体：
- `DelimiterNotFound`：没有找到指定的分隔符。
- `Io`：I/O操作错误。
- `Decoding`：解码错误。

这些结构体和枚举提供了一个通用的工具，可以方便地处理基于分隔符的数据流。

