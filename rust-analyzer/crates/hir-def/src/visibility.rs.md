# File: rust-analyzer/crates/hir-def/src/visibility.rs

在rust-analyzer的源代码中，`rust-analyzer/crates/hir-def/src/visibility.rs`文件的作用是定义了 Rust 语言中的可见性规则和相关的数据结构。

该文件中定义了以下两个 `enum` 类型：

1. `RawVisibility`：表示原始的可见性。它有以下几个成员：
   - `Public`：表示公开的可见性，即 `pub` 关键字修饰的部分。
   - `Private`：表示私有的可见性，即未使用 `pub` 关键字修饰的部分。
   - `Inherited`：表示继承的可见性，即根据所在上下文决定的可见性。
   - `Module`：表示模块级的可见性，即仅模块内可见。
   - `Super`：表示父级的可见性，即仅父级模块内可见。
   - `Crate`：表示 crate 级的可见性，即仅整个 crate 内可见。
   - `Restricted`：表示受限的可见性，具体规则可能由宏等决定。

2. `Visibility`：表示经过处理和解析后的可见性。它有以下几个成员：
   - `Public`：公开可见性。
   - `InModule`：在当前模块内可见。
   - `Restricted { path: Path }`：受限可见性，具体路径由 `Path` 类型指定。
   - `InSuperMod`：在父级模块内可见。
   - `InCrate`：在整个 crate 内可见。

这些可见性规则和数据结构用于编译器在进行代码解析和类型检查时确定标识符的可见范围。它们帮助编译器判断哪些代码、变量、函数等可以在不同的作用域内被访问和使用，从而遵循 Rust 语言的可见性约束。

