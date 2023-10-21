# File: cargo/src/cargo/util/toml/embedded.rs

在Rust Cargo中，cargo/src/cargo/util/toml/embedded.rs文件的作用是提供一个嵌入式的Toml解析器，用于解析Cargo.toml文件。

该文件定义了一些结构体和枚举类型，其中DocFragment结构体表示Toml文档的片段，可以包含键值对、内联表、内联数组，以及用于扩展的自定义键值对。它包含以下字段：
- key: 字符串类型，表示键值对的键。
- value: TomlValue类型，表示键值对的值，可以是基本数据类型、数组、表、内联表或内联数组。
- doc: 字符串类型，表示键值对的注释文档。
- position: (usize, usize)类型，表示键值对在原始文本中的位置。

CommentKind枚举类型表示注释的类型，包含以下几种：
- Line: 表示单行注释，以'#'字符开头。
- Block: 表示块注释，以'/*'开始，以'*/'结束。
- Doc: 表示文档注释，以'///'或'//!'开始。

embedded.rs文件中还定义了一些函数用于解析Toml文档，如parse_doc_fragment函数用于解析DocFragment对象，parse_literal函数用于解析字面量，parse_inline_comment函数用于解析行内注释，parse_value函数用于解析TomlValue对象等。

总的来说，cargo/src/cargo/util/toml/embedded.rs文件的作用是提供一个嵌入式的Toml解析器，用于解析Cargo.toml文件中的键值对、注释和文档片段。

