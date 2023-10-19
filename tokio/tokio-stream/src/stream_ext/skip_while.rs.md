# File: tokio/tokio-stream/src/stream_ext/skip_while.rs

在tokio源代码中，tokio-stream库的skip_while.rs文件中定义了SkipWhile流适配器。SkipWhile是一个结构体，用于创建一个新的流，该流会跳过满足指定条件的元素，直到遇到不满足条件的元素。

该文件中定义了以下结构体：

1. SkipWhile<St, Fut, F>：这是SkipWhile流适配器的主要结构体。它包含了原始流（Stream）的所有权（represented by St），一个可以处理流的未来（Future）类型（represented by Fut），以及一个用于判断是否跳过元素的闭包（represented by F）。

该结构体实现了Stream trait，并提供了下述方法：
- new(stream: St, f: F)：创建一个新的SkipWhile适配器，使用给定的原始流和判断闭包。
- poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>：尝试获取下一个元素，如果满足判断条件，则继续跳过，直到找到不满足条件的元素为止。
- size_hint(&self) -> (usize, Option<usize>)：返回适配器中剩余元素的数量估计。

2. SkipWhileFuture<Fut, F>：这是SkipWhile的未来类型结构体。它封装了一个未来对象以及一个判断闭包。
该结构体实现了Future trait，并提供了下述方法：
- new(future: Fut, skip: F)：创建一个新的SkipWhileFuture，使用给定的未来对象和判断闭包。
- poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Fut::Output>：获取特定未来对象的输出，并跳过满足判断条件的输出。

SkipWhile适配器允许用户在流（Stream）上应用“跳过”操作直到满足特定条件。该适配器返回一个新的流，该流会在遇到不满足条件的元素时停止跳过。

总之，tokio-stream中的skip_while.rs文件定义了SkipWhile流适配器和SkipWhileFuture未来类型结构体，用于在tokio中的流上执行拥有特定条件的“跳过”操作。

