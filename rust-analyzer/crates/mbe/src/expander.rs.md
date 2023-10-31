# File: rust-analyzer/crates/mbe/src/expander.rs

`expander.rs` 文件是 `mbe`（Macro By Example）的扩展器模块的实现文件。该模块负责实际执行宏展开操作。

`Bindings` 结构体和 `Binding`、`Fragment` 枚举是 `mbe` 模块中用于表示宏展开结果的数据结构。下面具体介绍它们的作用。

- `Bindings` 结构体是 `mbe` 模块的主要输出。它包含了完整的宏展开后的信息。它的字段包括：
  - `errors`: 一个错误列表，记录了在宏展开过程中发现的错误。
  - `bindings`: 一个 `Binding` 列表，表示宏展开后所有绑定的名称及其相应的 `Fragment`。
  - `body`: 展开后的宏体。

- `Binding` 枚举表示一个宏绑定（macro-binding），即将宏参数绑定到相应的宏体的名称和值。它的不同变体如下：
  - `Pat`：表示一个模式绑定，将宏参数绑定到另一个模式上。
  - `Expr`：表示一个表达式绑定，将宏参数绑定到另一个表达式上。
  - `Ty`：表示一个类型绑定，将宏参数绑定到另一个类型上。
  - `Visibility`：表示一个可见性绑定，将宏参数绑定到另一个可见性（visibility）上。

- `Fragment` 枚举包含了宏体中具体的展开内容，即将宏参数替换成实际的值后的文本片段。它的不同变体如下：
  - `Ast`：表示一个 AST 片段。
  - `MacroCall(t)`：表示一个宏调用，并包含了它的相关信息。
  - `Bindings(b)`：表示一个绑定片段，其中包含了绑定的名称和抽象语法树（AST）。

总体来说，`expander.rs` 文件定义了 `mbe` 模块的核心数据结构和宏展开的逻辑，负责将宏定义转换为宏调用时的具体展开结果。这些结构体和枚举通过其字段和变体，提供了对宏展开过程中的绑定和文本片段的表示和处理。
