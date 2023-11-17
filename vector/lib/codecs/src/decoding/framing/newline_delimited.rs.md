# File: vector/lib/codecs/src/decoding/framing/newline_delimited.rs

在Rust生态的vector项目中，`vector/lib/codecs/src/decoding/framing/newline_delimited.rs`文件的作用是实现了一个用于解码以换行符分隔的文本流的解码器。

具体来说，这个解码器是通过实现一个名为`NewlineDelimitedDecoder`的解码器结构体来实现的。`NewlineDelimitedDecoder`是一个泛型结构体，它使用`CharacterDelimitedDecoder`结构体来分割以换行符分隔的文本流。以下是这些结构体的详细说明。

1. `NewlineDelimitedDecoderConfig`：这是一个配置结构体，用于配置`NewlineDelimitedDecoder`的行为。它包含一些可选的属性，例如最大行长度、是否将行尾的换行符输出等。

2. `NewlineDelimitedDecoderOptions`：这是一个枚举类型，用于指定解码选项。它包含两个选项：`LineEnding`和`IncludeLastLineEnding`。`LineEnding`选项用于指定解析输入数据的行尾字符，可以是`\n`或`\r\n`。`IncludeLastLineEnding`选项用于指定是否将最后一行的行尾字符包含在解码的行中。

3. `NewlineDelimitedDecoder`：这是主要的解码器结构体。它实现了`Decoder` trait，该trait定义了解码器的核心功能。`NewlineDelimitedDecoder`内部使用`CharacterDelimitedDecoder`结构体来处理以换行符分隔的数据流。它接受一个泛型参数，用于指定输出的行类型。通过实现`Decoder` trait中的方法，如`decode`和`finish`，`NewlineDelimitedDecoder`可以解码输入的文本流并生成相应的行。

总结起来，`NewlineDelimitedDecoder`结构体及其相关的配置结构体用于实现以换行符分隔的文本流的解码器，并提供了一些可配置的选项，以满足不同的解码需求。

