# File: rust-clippy/clippy_utils/src/usage.rs

在 rust-clippy 项目中，`rust-clippy/clippy_utils/src/usage.rs` 文件的作用是收集和分析 Rust 代码中的变量使用情况。

`MutVarsDelegate` 结构体是一个 AST 访问器（Visitor），用于收集并记录可变变量的使用情况。它实现了 `Delegate` trait，可以通过实现该 trait 来访问和记录 AST 中的各种节点。`MutVarsDelegate` 用于在代码中查找可变变量的使用情况，并将结果保存在其内部的 `vars` 字段中。

`ParamBindingIdCollector` 结构体也是一个 AST 访问器，用于收集和记录函数参数的绑定（binding）的使用情况。它实现了 `Delegate` trait，并在访问 AST 中的函数参数时，将参数的绑定信息添加到其内部的 `param_binding_ids` 字段中。这个结构体的目的是为了在特定的代码检查中，比如检查未使用的函数参数，收集并提供函数参数的绑定信息。

`BindingUsageFinder` 结构体是另一个 AST 访问器，它的作用是查找和记录特定名称的绑定（binding）在代码中的使用情况。它实现了 `Delegate` trait，并在访问 AST 中的变量引用时，检查引用是否与目标绑定的名称相同，并将使用情况记录在其内部的 `usage` 字段中。

这些结构体都是辅助工具，用于在 lint 规则的实现中进行代码分析和收集信息。它们提供了访问和分析 AST 的能力，以便在代码检查过程中获取所需的信息，并进行相应的处理和报告。

