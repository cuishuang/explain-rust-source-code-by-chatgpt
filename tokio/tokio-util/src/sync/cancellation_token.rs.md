# File: tokio/tokio-util/src/sync/cancellation_token.rs

在Tokio中，tokio-util/cancellation_token.rs文件实现了“Cancellation Token”模式。Cancellation Token模式用于在异步任务中发送取消信号或等待取消信号，并相应地终止或解除阻塞任务。

在该文件中，定义了以下几个结构体：

1. `CancellationToken`：这是Cancellation Token的主要结构体。它可以用于创建和发送取消信号。Cancellation Token可以与多个`WaitForCancellationFuture`配对使用，在多个任务中共享取消信号。

2. `WaitForCancellationFuture<'a>`：这是一个用于等待取消信号的Future。它实现了`Future` trait，并可以被await或加入任务的执行上下文中。这个Future在Cancellation Token被取消时会返回。`'a`是Cancellation Token的生命周期。

3. `WaitForCancellationFutureOwned`：这是`WaitForCancellationFuture`的所有权版本。它实现了`Future` trait，并可以在不同的线程上执行。

首先，`CancellationToken`可以在异步任务中创建和发送取消信号。它的主要方法是`new()`，用于创建一个新的Cancellation Token。然后，可以使用`is_cancelled()`检查是否已取消，以及使用`cancel()`方法发送取消信号。

当Cancellation Token被取消时，与之相关联的`WaitForCancellationFuture`将返回带有取消状态的结果。

例如，可以创建一个Cancellation Token，并将其与多个异步任务共享。这些任务可以轮询检查Cancellation Token的状态，如果已取消，则终止任务。或者，使用`WaitForCancellationFuture`等待取消信号，然后在取消后执行某些操作。

`WaitForCancellationFuture`是一个Future，用于等待Cancellation Token被取消。它使用`poll()`方法检查Cancellation Token的状态，并返回一个表示取消状态的结果。

`WaitForCancellationFutureOwned`是`WaitForCancellationFuture`的所有权版本。它可以在线程之间传递，并在不同的执行上下文中等待Cancellation Token的取消。

综上所述，tokio-util/cancellation_token.rs文件的作用是实现Cancellation Token模式，用于在异步任务中发送和等待取消信号，并相应地终止或解除阻塞任务。

