# File: tokio/tokio-util/src/sync/poll_semaphore.rs

在tokio-util包的poll_semaphore.rs文件中定义了一个用于异步信号量的实现，该实现基于时钟计时器手动触发唤醒正在等待资源的任务。

这个文件的作用是实现一个基于Poll的信号量，它提供了一种资源管理机制，允许在并发任务中限制对共享资源的访问。

PollSemaphore文件中定义了几个struct：PollSemaphore、Permit、SpawnPermit、AcquireFuture和ReleaseFuture。

1. PollSemaphore: 这个struct是PollSemaphore的主要实现，代表一个信号量。它包含了一个内部状态，记录了信号量的当前可用资源数和等待资源的任务队列。

2. Permit: 这个struct表示获取到的资源的许可，它包含了一个Arc<Mutex<Option<PollSemaphore>>>，用来记录持有这个许可的任务执行完毕后要将许可返还给信号量。

3. AcquireFuture: 这个struct表示一个获取资源的异步操作，它实现了Future trait，代表了一个异步计算的结果。在调用`AcquireFuture.await`时，会等待信号量中资源变得可用，并返回一个Permit。

4. ReleaseFuture: 这个struct表示一个释放资源的异步操作，它实现了Future trait，代表了一个异步计算的结果。在调用`ReleaseFuture.await`时，会将Permit返还给信号量，使其可用资源数增加。

与传统的线程锁不同的是，PollSemaphore的实现是基于非阻塞式的异步I/O操作的，相比于阻塞式的线程锁，可以更好地集成到Tokio的异步编程模型中，提供更好的性能和可伸缩性。通过使用PollSemaphore，可以控制并发任务对共享资源的访问，避免资源竞争和冲突，确保代码的安全性和正确性。

