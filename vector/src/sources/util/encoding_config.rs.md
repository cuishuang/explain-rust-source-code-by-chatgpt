# File: vector/src/sources/util/encoding_config.rs

在Rust生态vector项目的源代码中，vector/src/sources/util/encoding_config.rs文件的作用是定义了用于配置数据编码的结构体和相关函数。

具体来说，这个文件定义了以下几个结构体：

1. `EncodingConfig`: 这个结构体是用于配置数据编码的主要结构体。它包含了以下字段：
   - `charset`: 指定字符集的字符串，可以是标准的字符集名称，例如"UTF-8"、"ISO-8859-1"等。
   - `use_bom`: 布尔值，表示是否需要在生成的数据文件中包含字节顺序标记（BOM）。
   - `line_ending`: 表示换行符的字符串，可以是"\n"、"\r\n"等。
   - `quote`: 指定引用字符的字符串，用于包裹字段值中的特殊字符。
   - `separator`: 指定字段分隔符的字符串。

2. `EncodingConfigBuilder`: 这个结构体是一个辅助结构体，用于构建和配置`EncodingConfig`结构体。它提供了一系列方法来设置`EncodingConfig`结构体的各个字段，并最终生成一个完整的`EncodingConfig`实例。

3. `Charset`: 这个结构体是一个枚举类型，表示字符集的不同选项。它包含了几个常见的字符集，如"UTF-8"、"ISO-8859-1"等。

这些结构体和相关函数的作用是为了方便用户配置数据的编码方式。通过使用`EncodingConfig`结构体和`EncodingConfigBuilder`辅助结构体，用户可以指定字符集、是否使用字节顺序标记、换行符、引用字符和字段分隔符等参数，以满足不同数据源及消费者的编码要求。同时，`Charset`枚举类型提供了一些常见字符集的选项，进一步简化了配置过程。

