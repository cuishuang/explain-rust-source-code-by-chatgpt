# File: vector/lib/vector-stream/src/futures_unordered_count.rs

在Rust生态vector项目的源代码中，`vector/lib/vector-stream/src/futures_unordered_count.rs` 文件的作用是实现了一个具有计数功能的`FuturesUnorderedCount` 结构体。这个结构体通过存储异步任务的 `FuturesUnordered`，并且跟踪活动任务的数量。

`FuturesUnordered` 是 `futures` 库提供的一个类型，用于存储和管理异步任务并发执行的结果。它可以同时运行多个异步任务，并在所有任务都完成时返回结果。

`FuturesUnorderedCount` 结构体通过扩展 `FuturesUnordered` 结构体，并添加了一个计数器字段来跟踪活动任务的数量。它实现了 `Stream` trait，并为外部使用者提供异步任务的迭代器。该结构体允许使用者在迭代异步任务的同时，跟踪活动任务的数量，以便在需要时进行相应处理。

在 `FuturesUnorderedCount` 结构体中，有如下几个重要的结构体的定义：

1. `FutEntry`: 这个结构体用于表示每个异步任务的条目。它包含一个 `FuturesUnordered` 的条目，并且可以通过 `count_stream()` 方法来取得自身的生命周期。
2. `TaskCompletion`: 这个结构体用于表示异步任务的完成状态。它包含一个 `bool` 类型的字段，表示收到了任务的完成信号。
3. `TaskCompletionRx`: 这个结构体是一个 `Vec`，用于存储多个异步任务的完成状态。它可以通过 `new` 方法来创建，并且通过调用 `complete_all()` 方法来等待所有任务完成。

通过使用这些结构体，`FuturesUnorderedCount` 实现了以下功能和特点：

- 可以使用 `FuturesUnorderedCount::new()` 方法来创建一个新的实例，并自动创建一个 `FuturesUnordered` 实例。
- 可以通过 `FuturesUnorderedCount::next()` 方法获取下一个异步任务的结果，同时更新活动任务的计数器。
- 可以使用 `FuturesUnorderedCount::count()` 方法获取当前活动任务的数量。
- 可以使用 `FuturesUnorderedCount::join_all()` 方法等待所有异步任务完成，并返回一个包含所有任务结果的 `Vec`。
- 可以使用 `FuturesUnorderedCount::cancel()` 方法取消所有运行中的异步任务。

总之，`FuturesUnorderedCount` 结构体扩展了 `FuturesUnordered`，为异步任务的执行和处理提供了更加方便的接口，并且能够跟踪和控制活动任务的数量。

