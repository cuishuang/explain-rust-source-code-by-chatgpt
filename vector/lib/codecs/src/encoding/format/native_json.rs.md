# File: vector/lib/codecs/src/encoding/format/native_json.rs

在Rust ecosystem中，vector是一个高性能、可扩展的日志收集工具。vector项目的源代码中，`native_json.rs`文件位于`vector/lib/codecs/src/encoding/format`路径下，该文件的作用是实现对原生Json格式的编码和序列化。

`NativeJsonSerializerConfig`是一个struct，用于配置原生Json序列化器的参数。它定义了一些字段，如`precision`，`strict`，`timestamp_format`等，用于指定序列化器的行为。

`NativeJsonSerializer`也是一个struct，是原生Json格式的序列化器。它实现了`EventSerializer`trait中的方法，用于将事件序列化为原生Json格式的字节流。`NativeJsonSerializer`结构体中包含了一个名为`config`的字段，类型为`NativeJsonSerializerConfig`，用于确定序列化器的配置。

通过配置`NativeJsonSerializerConfig`，我们可以根据需要定制原生Json序列化器的行为，例如指定小数的精度、启用严格模式以强制符合Json规范、定义时间戳的格式等。然后，使用`NativeJsonSerializer`结构体的方法将事件对象序列化为对应的原生Json格式的字节流。

总之，`native_json.rs`文件中的`NativeJsonSerializerConfig`结构体和`NativeJsonSerializer`结构体分别用于配置和实现原生Json格式的编码和序列化。

