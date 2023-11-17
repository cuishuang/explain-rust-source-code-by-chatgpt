# File: rust-clippy/clippy_utils/src/numeric_literal.rs

在rust-clippy的源代码中，`numeric_literal.rs`文件位于`rust-clippy/clippy_utils/src`目录中，它的作用是用于处理数字字面量的操作。

`NumericLiteral<'a>`是一个结构体，用于表示数字字面量的信息。它有以下字段：

- `source`：保存数字字面量的原始字符串；
- `digits`：保存去除数字字面量后缀的纯数字字符串；
- `suffix`：保存数字字面量的后缀字符串；
- `radix`：保存数字字面量的进制。

`NumericLiteral`结构体提供了一些方法，用于解析数字字面量的不同部分，并判断数字字面量的类型。

`Radix`是一个枚举类型，用于表示数字字面量的进制。它有以下几个变体：

- `Binary`：表示二进制（以`0b`或`0B`开头）；
- `Octal`：表示八进制（以`0o`或`0O`开头）；
- `Decimal`：表示十进制（没有前缀，或以`0`~`9`开头）；
- `Hexadecimal`：表示十六进制（以`0x`或`0X`开头）。

通过将进制信息保存在`Radix`枚举中，可以在处理数字字面量时更方便地获取其进制，并对它进行进一步操作或检查。

