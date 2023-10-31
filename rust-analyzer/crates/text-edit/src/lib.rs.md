# File: rust-analyzer/crates/text-edit/src/lib.rs

在rust-analyzer的源代码中，`rust-analyzer/crates/text-edit/src/lib.rs`文件的作用是提供关于文本编辑的功能。该文件中定义了三个结构体：`Indel`、`TextEdit`和`TextEditBuilder`。

1. `Indel`结构体用于表示一次文本修改操作。它包含两个字段：`new_text`和`delete`。`new_text`表示要插入或替换的新文本，`delete`表示要删除的文本范围。通过在文本的特定位置指定`Indel`实例，可以进行插入、删除或替换文本的操作。

2. `TextEdit`结构体用于表示一组文本修改操作。它包含一个 `Vec<Indel>` 字段，即一个 `Indel` 的集合，用于按顺序保存一系列编辑操作。通过将多个 `Indel` 实例放在 `TextEdit` 中，可以按顺序执行一系列的编辑操作。

3. `TextEditBuilder`结构体用于构建 `TextEdit` 的实例，通过将一系列 `Indel` 实例添加到 `TextEditBuilder` 中，最终可以生成一个完整的 `TextEdit`。可以通过调用 `TextEditBuilder` 的 `insert()`、`replace()` 或 `delete()` 方法来创建 `Indel` 实例，并将其添加到 `TextEditBuilder` 中。

这些结构体的作用是为了简化和封装对文本的编辑操作。通过使用 `Indel`、`TextEdit` 和 `TextEditBuilder`，可以方便地表示和管理一系列的文本修改操作，并将它们应用于源代码中。这对于代码分析、重构和编辑工具非常有用。

