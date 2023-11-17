# File: vector/lib/codecs/src/decoding/format/native_json.rs

在Rust生态中，vector是一款用于可靠、可扩展和高性能数据传输的工具。vector项目的代码仓库中，vector/lib/codecs/src/decoding/format/native_json.rs这个文件是用于处理本地JSON格式的解码工具。

该文件中定义了三个struct：NativeJsonDeserializerConfig、NativeJsonDeserializerOptions和NativeJsonDeserializer。

1. NativeJsonDeserializerConfig：
   - 该struct用于配置本地JSON解码器的行为。
   - 它包含了一些字段来控制解码过程。例如，可以设置是否采用strict模式解析JSON，设置最大的json解析深度，定义json解析期间的缓冲区大小等。这些配置选项可以灵活地根据需求进行调整，以满足不同的解码方式。

2. NativeJsonDeserializerOptions：
   - 该struct是NativeJsonDeserializer的构造参数。
   - 它包含了一些配置参数，用于创建NativeJsonDeserializer实例时传递给构造函数。例如，可以指定解码器的配置、输入的数据源等。

3. NativeJsonDeserializer：
   - 该struct是具体的本地JSON解码器。
   - 它实现了Deserializer trait，用于将JSON数据流转换为Rust数据类型。
   - NativeJsonDeserializer通过读取输入的JSON数据流，并按照配置的规则进行解析，最终将解析结果转换为合适的数据类型。
   - 解析过程中，NativeJsonDeserializer可以处理不同的JSON数据类型，如null、bool、number、string、array和object。并且可以处理嵌套的数据结构，以及复杂的字段映射关系和转换规则。

总的来说，vector/lib/codecs/src/decoding/format/native_json.rs文件中的NativeJsonDeserializerConfig、NativeJsonDeserializerOptions和NativeJsonDeserializer这些struct定义了对本地JSON格式进行解码的配置选项和工具。这些组件可以帮助用户通过定制化的方式解析本地的JSON数据，并将其转换为Rust数据类型，以便进一步处理和分析。

