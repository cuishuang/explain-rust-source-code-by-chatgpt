# File: miri/src/borrow_tracker/tree_borrows/diagnostics.rs

在 Rust 的 miri 项目中，`miri/src/borrow_tracker/tree_borrows/diagnostics.rs` 文件的作用是为树形借用追踪器（borrow tracker）提供诊断信息。下面将详细介绍每个 struct 和 enum 的作用：

1. `Event`: 用于记录每个操作的事件，包括借用开始、借用结束和修改。

2. `History`: 为每个借用创建一个独一无二的历史记录，用于追踪每个借用发生的位置和借用的操作。

3. `HistoryData`: 维护一组历史记录，将每个历史记录与特定的节点相关联。

4. `NodeDebugInfo`: 为节点提供调试信息，包括节点的类型、状态和操作。

5. `TbError`: 表示树形借用追踪器的错误，并提供与错误相关的信息。

6. `DisplayFmtWrapper`: 封装不同数据类型的展示格式。

7. `DisplayFmtPermission`: 展示借用的权限。

8. `DisplayFmtPadding`: 控制展示格式的缩进。

9. `DisplayFmtAccess`: 展示借用的访问类型。

10. `DisplayFmt`: 定义了要展示的元素的格式，包括借用盒子的显示和归属关系的显示。

11. `DisplayIndent`: 展示缩进的设置。

12. `DisplayRepr`: 展示 Rust 数据类型的格式。

这些 struct 是为了方便展示和调试树形借用追踪器中的数据而创建的。

以下是 enum 类型的作用：

1. `AccessCause`: 表示访问操作的原因，包括共享借用、可变借用和移动。

2. `TransitionError`: 表示状态转换时可能出现的错误，包括非法的借用时间和无效的节点引用。

这些 enum 提供了更具体的错误和状态转换的类型信息，可以在树形借用追踪器中使用。

