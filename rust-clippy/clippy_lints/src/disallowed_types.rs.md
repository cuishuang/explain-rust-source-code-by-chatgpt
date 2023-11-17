# File: rust-clippy/clippy_lints/src/disallowed_types.rs

在rust-clippy的源代码中，`disallowed_types.rs` 文件的作用是定义了 Clippy 的几个 lint 规则，用于检测并报告禁止使用的类型。

具体来说，该文件中定义了一个名为 `DisallowedTypes` 的结构体，其作用是存储了一些禁止使用的类型的信息，并提供了一些方法来进行类型的检查和报告。

`DisallowedTypes` 结构体的几个字段分别用于表示不同类型的禁止使用信息。下面是每个字段的作用：

1. `disallowed_types: HashSet<DisallowedType>`：该字段是一个 `HashSet`，用于存储所有的禁止使用的类型。每个 `DisallowedType` 结构体表示了一个禁止使用类型的详细信息，包括类型的名称、错误消息等。
2. `pub struct DisallowedType`：定义了一个 `DisallowedType` 结构体，用于表示一个禁止使用的类型。该结构体有几个字段，包括：
   - `type_str: &'static str`：表示禁止使用的类型的名称。
   - `err_msg: &'static str`：表示在检测到禁止使用类型时的错误消息，用于报告给用户。
   - `is_trait_impl: bool`：表示禁止使用的类型是否是 trait 的实现。
   - `is_unknown: bool`：表示禁止使用的类型是否是一个未知的类型。

`DisallowedTypes` 结构体还提供了一些方法来进行类型的检查和报告，包括：

1. `fn from_str(type_str: &str) -> Option<DisallowedType>`：根据给定的类型名称，创建一个对应的 `DisallowedType` 结构体实例，如果类型名称存在于 `disallowed_types` 中，则返回 Some(DisallowedType)；否则返回 None。
2. `fn check_disallowed_types(&self, cx: &LateContext<'_>, ty: Ty<'_>, span: Span)`：检查给定类型 `ty` 是否存在于 `disallowed_types` 中，如果存在则报告错误消息。
3. `fn is_disallowed_path(&self, cx: &LateContext<'_>, path: &Path) -> bool`：检查给定的路径 `path` 是否表示了禁止使用的类型。
4. `fn check_trait_impl(&self, cx: &LateContext<'_>, item: &Item<'_>)`：检查给定的 trait 实现是否是禁止使用的类型，并报告错误消息。
5. `fn check_unused_type(&self, cx: &LateContext<'_>, item: &Item<'_>)`：检查给定的未使用的类型是否是禁止使用的类型，并报告错误消息。

这些方法通过遍历 AST（抽象语法树）和使用 Clippy 的 lint 框架来进行类型的检查和报告，以帮助开发者遵守规范和最佳实践。

