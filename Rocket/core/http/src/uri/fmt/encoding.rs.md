# File: Rocket/core/http/src/uri/fmt/encoding.rs

在Rocket web框架的源代码中，`encoding.rs`文件是实现了URI的编码和解码功能。URI（Uniform Resource Identifier）是用于标识资源的字符串，由多个部分组成，包括协议、主机名、路径等。编码和解码URI主要是为了处理URI中包含的非法字符或特殊字符。

`unsafe_encode_set`、`P`、`encode_set`、`DEFAULT_ENCODE_SET`这些结构体和 trait 是 URI 编码和解码的关键组件。

- `unsafe_encode_set` 是一个默认的不安全编码集，它包含用于 URI 编码的特殊字符和保留字符。这些字符在编码时，不需要进行百分号编码。这个编码集主要用于标准化的 URI 编码。
- `P` 是一个泛型参数，表示编码集的类型。
- `ENCODE_SET<P: EncodeSet>` 是一个泛型结构体，它包含了对应编码集 `P` 中的字符集信息。这个结构体能够对 URI 中的字符进行编码。
- `DEFAULT_ENCODE_SET` 是一个默认的编码集，它实现了 `EncodeSet` trait 并提供了标准的 URI 编码。它主要用于默认情况下对 URI 进行编码。

`EncodeSet` 是一个 trait，它定义了编码和解码 URI 字符的方法。具体来说，它包括以下功能：
- `contains` 方法用于判断一个字符是否属于编码集。
- `encode_byte` 方法用于将一个字节编码成对应的百分号编码。
- `encode_char` 方法用于将一个字符编码成对应的百分号编码。
- `decode_byte` 方法用于将一个百分号编码的字节解码成对应的字符。

通过使用这些结构体和 trait，Rocket 实现了对 URI 的编码和解码操作，确保了 URI 的准确性和正确性。

