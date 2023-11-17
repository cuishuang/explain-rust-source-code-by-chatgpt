# File: rust-clippy/clippy_lints/src/manual_non_exhaustive.rs

在rust-clippy的源代码中，`rust-clippy/clippy_lints/src/manual_non_exhaustive.rs`这个文件的作用是实现了对 Rust 手动非穷尽类型的 lint 检查。

手动非穷尽类型是通过使用 `#[non_exhaustive]` 属性来修饰的，表示该类型是有意留有空间以便将来添加更多的可能性。手动非穷尽类型的引入使得对该类型的模式匹配时，Rust 编译器不再要求模式匹配覆盖所有可能的变体，从而在向类型添加新的变体时不会破坏已有代码。

该文件中定义了两个结构体，分别是`ManualNonExhaustiveStruct`和`ManualNonExhaustiveEnum`。这两个结构体用于在 lint 检查时辅助分析手动非穷尽类型。

`ManualNonExhaustiveStruct`结构体用于保存手动非穷尽的结构体的信息。它包含以下字段：
- `fields`：存储结构体的字段信息。
- `is_source_of_unknown_fields`：表示结构体是否是引起未知字段的源头。
- `is_sort_of_non_exhaustive`：表示结构体是否是某个手动非穷尽的变体。
- `been_seen`：表示手动非穷尽的结构体是否已被处理。

`ManualNonExhaustiveEnum`结构体用于保存手动非穷尽的枚举的信息。它包含以下字段：
- `variants`：存储枚举的变体信息。
- `been_seen`：表示手动非穷尽的枚举是否已被处理。

这两个结构体提供了一种在 lint 检查中跟踪手动非穷尽类型的方式，以便进行相关的分析和处理。这样可以确保在对手动非穷尽类型进行模式匹配时，覆盖了所有可能的变体，并且能够及时发现可能需要更新的地方。

