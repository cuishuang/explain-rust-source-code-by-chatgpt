# File: vector/lib/vector-common/src/finalization.rs

在Rust生态中的vector项目中，`finalization.rs`文件是用来定义一些与事件完成相关的结构体、枚举和trait的。

首先，`EventFinalizers(Vec<Arc<EventFinalizer>>)`是一个结构体，用于包含多个`EventFinalizer`的`Arc`（引用计数智能指针）。

`EventFinalizer`是一个trait，它定义了一个`finalize`方法，用于在处理事件后执行一些最终化的操作。

`BatchStatusReceiver(oneshot::Receiver<BatchStatus>)`是一个结构体，它包含一个`oneshot::Receiver`，可以接收到关于批处理状态的消息。

`BatchNotifier(Arc<OwnedBatchNotifier>)`是一个结构体，它包含一个`Arc`指向`OwnedBatchNotifier`。

`OwnedBatchNotifier`是一个结构体，它用于通知批处理的所有者批处理的状态。

`AddBatchNotifier`是一个trait，它定义了一个`add_batch_notifier`方法，用于向批处理添加通知器。

`Finalizable`是一个trait，它定义了一个`finalize`方法，用于在对象生命周期结束时执行一些操作。

`BatchStatus`和`EventStatus`都是枚举，用于表示批处理和事件的状态，包含了一些可能的状态值。

这些结构体、枚举和trait的定义在`finalization.rs`文件中，用于实现事件完成相关的功能，并且使得代码更易维护和组织。

