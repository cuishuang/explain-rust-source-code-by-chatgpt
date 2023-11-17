# File: rust-clippy/clippy_lints/src/matches/manual_utils.rs

文件`manual_utils.rs`位于`rust-clippy/clippy_lints/src/matches`目录下，它是 Rust Clippy 项目中的一个用于匹配表达式的辅助工具文件。

主要任务：
1. 提供了一系列的结构体和枚举，用于表示匹配表达式中的不同部分和模式。
2. 包含了一些辅助函数，用于处理匹配表达式。

详细介绍以下结构体和枚举：

1. `SuggInfo<'a>`：该结构体用于表示建议信息。其中包含着需要在建议时展示给用户的相关信息，例如建议的描述、代码片段等。

2. `SomeExpr<'tcx>`：该结构体用于表示某个表达式。主要包含了表达式的节点以及类型信息。

3. `OptionPat<'a>`：该枚举用于表示匹配表达式中的`Option`类型模式。包含以下几个变体：
   - `NonePat`：表示匹配`None`的模式。
   - `SomePat(pattern)`：表示匹配`Some`的模式，并包含了一个子模式`pattern`。
   - `AnyPat`：表示匹配任何`Some`值的模式。

以上这些结构体和枚举的作用是为 Rust Clippy 提供了细粒度的表示匹配表达式的工具，这些工具可以在 lints 的实现中使用，实现对匹配表达式的检查和建议。

