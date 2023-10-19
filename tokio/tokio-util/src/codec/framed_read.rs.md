# File: tokio/tokio-util/src/codec/framed_read.rs

tokio-util是Tokio的扩展组件库之一，提供了一些与I/O相关的工具和实用程序。framed_read.rs文件是tokio-util中的一个模块，它定义了用于读取I/O流并按照帧进行解析的相关类型和方法。

该文件中定义了以下几个结构体：
1. FramedRead<T, D>: 从I/O流中读取帧并解析的异步流。
   - T是实现AsyncRead trait的类型，表示要读取的I/O流。
   - D是实现Decoder trait的类型，表示用于解析帧的解码器。

2. FramedRead2<T, D, E>: 类似于FramedRead<T, D>，但还接受错误类型E，用于处理解析帧时可能出现的错误。

3. DecoderResult<T>: 解码器的结果类型，表示解析一个帧的结果。其中T是解析后的帧数据的类型。

FramedRead<T, D>结构体的作用是将一个实现了AsyncRead trait的类型和一个实现了Decoder trait的类型组合在一起，提供一个异步流接口用于读取帧并解析它们。它实现了Stream和AsyncRead traits，使其可以像流一样进行读取操作，并为解析出的每个帧提供一个Future。

使用FramedRead，可以将流切分为连续的帧，并使用提供的解码器对每个帧进行解析。解码器负责在I/O流中查找帧的起始和结束位置，并提供对应帧的解析方法。通过使用FramedRead，可以有效地处理基于帧的协议，如TCP或UDP中的消息传递。

FramedRead2与FramedRead类似，但它还接受一个错误类型E，用于处理解析帧时可能出现的错误。这使得在解析帧时可以返回不同类型的错误而不仅仅是设置解析失败。

总之，framed_read.rs文件中的结构体和方法提供了解析帧的工具，使得在异步I/O中处理基于帧的协议变得更加方便和高效。

