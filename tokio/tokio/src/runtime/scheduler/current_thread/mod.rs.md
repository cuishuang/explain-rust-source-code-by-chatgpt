# File: tokio/tokio/src/runtime/scheduler/current_thread/mod.rs

在tokio源代码中，tokio/tokio/src/runtime/scheduler/current_thread/mod.rs文件的作用是定义了当前线程调度器的实现。

具体而言，该文件定义了以下几个结构体：

1. `CurrentThread`：表示当前线程调度器，负责在当前线程上运行任务，并处理相关的事件。它实现了`Scheduler` trait，用于协调和管理任务的执行。它持有一个`Core`实例。

2. `Handle`：是`CurrentThread`的句柄，允许在外部线程中向`CurrentThread`提交任务执行和调度。可以通过`Handle`实例获取`Shared`实例。

3. `Core`：是`CurrentThread`的核心结构，负责管理任务队列、运行任务和处理事件等。它持有一个`Context`实例，并具有`run`方法用于启动当前线程的运行时。

4. `Shared`：是`CurrentThread`的共享数据结构，可以在执行上下文之间共享状态。可以通过`Shared`实例克隆新的`Context`实例，并在不同的执行上下文之间共享状态。

5. `Context`：表示任务的执行上下文，其中包含要执行的任务和可能的等待发送事件的唤醒器。每个`Context`实例被分配给一个任务，并在任务执行期间持有对`CurrentThread`的引用。

6. `CoreGuard<'a>`：是`CurrentThread`的核心保护结构，它在`run`方法运行期间持有对`Core`的引用，确保只有一个线程可以运行 `run` 方法。当`CoreGuard`实例离开作用域时，就会释放对`Core`的引用。

总之，`CurrentThread`实现了一个基于当前线程的任务调度器，可以在当前线程上运行由`Handle`提交的任务。`Core`负责管理任务队列和事件处理。`Shared`用于在不同的执行上下文之间共享状态。`Context`表示任务的执行上下文。`CoreGuard`保护核心运行时的同步。

