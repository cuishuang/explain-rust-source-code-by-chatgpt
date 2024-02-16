# File: /Users/fliter/rust-contribute/rustfmt/src/config/macro_names.rs

在Rust的rustfmt项目中，位于`rustfmt/src/config/macro_names.rs`的文件主要用于处理和存储与宏命名相关的配置信息。下面将详细介绍该文件中各个结构体（struct）和枚举（enum）的作用：

1. `MacroName(String)`：该结构体用于存储单个宏的名称，通过一个字符串进行表示。它的作用是作为宏名称的封装类型，方便在代码中处理和传递宏名称的信息。

2. `MacroSelectors(pub struct)`：该结构体对应于一个具体的宏选择器，它描述了选择宏使用的规则。它包含以下字段：
   - `exact`：一个 `Vec<MacroName>` 类型的字段，用于存储需要精确匹配的宏名称列表。
   - `wildcard`：一个 `Option<MacroName>` 类型的字段，用于存储需要进行通配符匹配的宏名称。

   通过使用 `MacroSelectors`，可以灵活地定义对特定宏的选择策略。

3. `MacroSelector`：该枚举用于表示宏选择器的不同状态。它有以下几个变体：
   - `All`：表示匹配所有宏。
   - `Selective`：表示具体的宏选择器，包含一个 `MacroSelectors` 类型的字段。
   - `None`：表示没有宏选择器，即未指定任何宏名称。

   `MacroSelector` 枚举用于在代码中表示宏选择器的不同状态，并提供了相应的操作和逻辑。

4. `MacroSelectorsError`：该枚举用于表示处理宏选择器时可能发生的错误。它有以下几个变体：
   - `InvalidMacroName(String)`：表示提供的宏名称无效。
   - `InvalidMacroSelector(String)`：表示宏选择器无效。

   `MacroSelectorsError` 枚举用于在处理宏选择器时，标识可能发生的错误类型，并提供相应的错误信息。

总结：`rustfmt/src/config/macro_names.rs` 文件中的结构体和枚举主要用于处理和存储与宏命名相关的配置信息，提供了对宏选择和匹配进行灵活定义的功能，并用于在代码中表示和处理宏选择器的不同状态和错误情况。

