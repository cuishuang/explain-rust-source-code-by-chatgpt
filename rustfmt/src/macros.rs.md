# File: /Users/fliter/rust-contribute/rustfmt/src/macros.rs

在Rust的rustfmt项目的源代码中，文件`macros.rs`位于路径`/Users/fliter/rust-contribute/rustfmt/src/macros.rs`，是用于处理宏（macros）相关的功能的模块。

**ParsedMacroArg** 结构体是解析后的宏参数的表示。它包含了宏参数的名字和解析后的值。

**MacroArgParser** 结构体负责解析宏参数的字符串表示。它提供了一个从字符串解析出 `ParsedMacroArg` 的方法。

**MacroParser** 结构体用于解析宏定义的字符串表示，并且生成 `Macro` 结构体实例。

**Macro** 结构体表示一个宏定义。它包含了宏的名称、参数列表、展开后的代码和所有相关信息。

**MacroBranch** 结构体表示宏的一个分支。它包含了一个匹配模式和对应的宏代码。

**MacroPosition** 枚举类型表示宏代码在输入中的位置。它有三个可能的值：`Start` 代表宏展开的开头位置， `Within` 代表宏展开的中间位置，`End` 代表宏展开的结尾位置。

**MacroArg** 枚举类型表示宏参数的不同类型。它有两个可能的值：`Text` 代表文本类型的宏参数，`Macro` 代表宏类型的参数。

**MacroArgKind** 枚举类型表示宏参数的种类。它有三个可能的值：`Ident` 代表标识符类型的宏参数，`Delimited` 代表包裹在分隔符中的宏参数，`Literal` 代表字面量类型的宏参数。

**SpaceState** 枚举类型表示空格的状态。它有四个可能的值：`NoSpace` 代表无空格状态，`MayBreak` 代表可能插入换行的状态，`MustBreak` 代表必须插入换行的状态，`MustFuse` 代表必须合并的状态。

这些结构体和枚举类型一起提供了对宏的解析、展开和格式化的功能，以确保代码在格式化后的输出中保持一致和美观。

