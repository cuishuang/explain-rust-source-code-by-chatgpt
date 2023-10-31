# File: rust-analyzer/crates/hir-def/src/per_ns.rs

在rust-analyzer的源代码中，`per_ns.rs` 文件位于 `rust-analyzer/crates/hir-def/src` 目录下。它定义了 `PerNs` 结构体和 `Namespace` 枚举，并提供了一种表示命名空间的机制。

`PerNs` 结构体代表一组从命名空间到可能的实体的映射集合。它具有以下字段：

- `types: Option<DefId>`：表示类型级别的实体，如结构体、元组、枚举等。
- `values: Option<DefId>`：表示值级别的实体，如函数、变量等。
- `macros: Option<DefId>`：表示宏定义。
- `is_ambiguous: bool`：表示是否存在多个可能的实体。

`PerNs` 结构体的作用是提供一个简单而灵活的方式来存储和检索命名空间中的实体，并跟踪命名空间的歧义性。

`Namespace` 枚举表示不同的命名空间，它有以下成员：

- `Types`：类型命名空间，包含表示类型级别实体的定义。
- `Values`：值命名空间，包含表示函数、变量等值级别实体的定义。
- `Macros`：宏命名空间，包含表示宏定义的定义。

`Namespace` 枚举的作用是提供一种分类和区分不同类型的实体，以便在 `PerNs` 结构体中进行组织和存储。通过使用不同的命名空间，可以更好地表示和检索代码中的不同类型的实体。

总而言之，`per_ns.rs` 文件定义了 `PerNs` 结构体和 `Namespace` 枚举，提供了一种表示命名空间中实体的机制。`PerNs` 结构体用于存储和检索命名空间中的实体集合，而 `Namespace` 枚举用于分类和区分不同类型的实体。

