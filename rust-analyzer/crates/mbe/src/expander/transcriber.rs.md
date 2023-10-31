# File: rust-analyzer/crates/mbe/src/expander/transcriber.rs

在rust-analyzer的源代码中，`rust-analyzer/crates/mbe/src/expander/transcriber.rs` 文件是宏展开器的一部分，用于处理宏展开的文本转录和嵌套状态的跟踪。

该文件定义了两个重要的 struct：`NestingState` 和 `ExpandCtx<'a>`。

`NestingState` 结构体用于跟踪宏展开嵌套的状态。在宏展开过程中，可能存在多个嵌套的宏调用。每个嵌套的宏调用都会创建一个新的 `ExpandCtx`，因此 `NestingState` 结构体维护了一个堆栈，用于记录每个嵌套的宏调用的状态。它包含了堆栈的深度信息以及嵌套的宏调用的位置信息。

`ExpandCtx<'a>` 结构体是宏展开器的上下文，用于记录宏展开的状态和相关数据。该结构体包含了一个 `NestingState` 对象，用于跟踪宏展开的嵌套状态。它还包含了一些其他的字段，例如源代码的语法树和索引，用于在宏展开过程中进行语法分析和绑定解析。`ExpandCtx` 结构体还提供了一些方法，如 `attr_expand` 和 `path_expand`，用于处理宏展开期间的属性和路径。

总体而言，`rust-analyzer/crates/mbe/src/expander/transcriber.rs` 文件定义了宏展开器的核心逻辑。通过这些结构体和方法，它能够实现宏展开的文本转录和嵌套状态的跟踪。

