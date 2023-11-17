# File: vector/lib/codecs/src/decoding/format/gelf.rs

文件`gelf.rs`是Rust生态vector项目中vector-lib库的一部分，用户定义了与Gelf格式相关的解码器。

- `GelfDeserializerConfig`结构体：表示Gelf解码器的配置。它包含一组字段，用于配置解码器的行为，例如解析Gelf消息时使用的字符串字段名称、Gelf版本号等。

- `GelfDeserializerOptions`结构体：定义了Gelf解码器的选项。它包含一组字段，用于控制解码器的行为，例如是否进行字段名称映射、是否支持环境变量配置等。

- `GelfDeserializer`结构体：实现了Gelf解码器的逻辑。它包含了解码Gelf消息的方法，该方法接收一个字节流作为输入，并返回一个解码后的Gelf消息。解码器将根据配置和选项对Gelf消息进行解析，最终生成一个`GelfMessage`实例。

- `GelfMessage`结构体：表示解码后的Gelf消息对象。它包含了Gelf消息的各个字段，例如主机名、应用程序名称、日志级别、时间戳、完整的消息内容等。

这些结构体共同实现了Gelf解码器的功能，用户可以使用这些结构体进行Gelf消息的解码和处理。例如，用户可以创建一个`GelfDeserializerConfig`实例，配置解码器的行为，然后使用`GelfDeserializer`对Gelf消息进行解码，并使用生成的`GelfMessage`对象进行后续处理和分析。

