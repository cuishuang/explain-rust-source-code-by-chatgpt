# File: rust-clippy/clippy_lints/src/non_send_fields_in_send_ty.rs

在rust-clippy的源代码中，rust-clippy/clippy_lints/src/non_send_fields_in_send_ty.rs文件的作用是实现一个lint（即代码检查工具），用于检查在Send类型中存在不可发送字段的情况。

在这个文件中，有两个结构体：NonSendFieldInSendTy和NonSendField<'tcx>。

1. NonSendFieldInSendTy 结构体是一个可提示错误信息的包装器，用于表示在Send类型中存在不可发送字段的情况。它包含了一个存储类型的字段，并实现了Debug和Display trait，以便进行错误信息的打印。

2. NonSendField<'tcx> 结构体表示不可发送的字段。它包含了字段的名称和字段的类型，并实现了Debug和Display trait，以便进行错误信息的打印。

这两个结构体都是用于在代码检查中记录不可发送字段的信息，并提供错误提示。通过这些结构体，rust-clippy可以在静态分析代码的过程中检测到潜在的线程不安全问题，并给出相应的警告或错误信息，帮助开发者避免潜在的并发问题。

