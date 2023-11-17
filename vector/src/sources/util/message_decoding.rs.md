# File: vector/src/sources/util/message_decoding.rs

文件 `message_decoding.rs` 在 Rust 生态 `vector` 项目中的 `vector/src/sources/util` 目录下。该文件的作用是实现消息解码功能，用于将接收到的原始字节流解码为可读的消息。

详细介绍在以下结构体和 trait:

1. `FoldFinally<I>` 结构体: 该结构体是一个迭代器适配器，它包装了一个迭代器 `I`，提供了用于处理迭代器元素的函数 `fold_finally`。结构体内部还包含一个枚举类型 `FoldState` 用于表示 `fold_finally` 函数的运行状态。

2. `FoldFinallyExt` trait: 该 trait 提供了许多针对 `FoldFinally<I>` 结构体的方法扩展。这些方法包括 `find_index`, `last_success`, `last_index` 等用于处理 `FoldFinally` 的迭代器适配器。

   - `find_index` 方法用于在 `FoldFinally` 的迭代过程中查找符合条件的索引。
   - `last_success` 方法则在最后一个成功的迭代元素处返回结果。
   - `last_index` 方法返回最后一个成功迭代的索引。

总之，`message_decoding.rs` 文件的作用是提供用于消息解码的结构体和相关函数，以及对 `FoldFinally<I>` 结构体的扩展方法，方便开发者在解码过程中处理迭代器的元素。

