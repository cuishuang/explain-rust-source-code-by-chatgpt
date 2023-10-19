# File: tokio/tokio/src/loom/mocked.rs

在Tokio源代码中，`tokio/tokio/src/loom/mocked.rs`文件的作用是提供了一个用于测试的模拟异步运行时。它使用了`loom`库来创建一个与实际的异步运行时相似的模拟版本，以便进行并发性和可靠性的测试。

`MockedYield`结构体是用于模拟异步运行时中`task::yield_now()`函数的实现。这个函数会暂停当前任务并允许其他任务运行。`MockedYield`结构体会记录任务的暂停和恢复，并提供相关的方法进行模拟。

`FutureCell<T>`结构体是一个包装器，用于记录特定类型T的值。当一个`FutureCell`被包装在`MockYield`中时，它会追踪对值的访问和修改，以便进行模拟。

`Mutex<T>(loom::sync::Mutex<T>)`结构体是一个互斥锁类型的封装。它通过调用`loom`库中的互斥锁类型来创建一个互斥锁，以确保在并发环境下的线程安全性。

