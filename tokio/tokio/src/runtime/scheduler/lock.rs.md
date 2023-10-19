# File: tokio/tokio/src/runtime/scheduler/lock.rs

在Tokio的源代码中，tokio/tokio/src/runtime/scheduler/lock.rs 文件是用于实现 Tokio 调度器中的锁的。

该文件中实现了三个 trait：Lock、OwnedLock 和 AwaitLock。

1. Lock<T> trait：定义了锁的基本操作。它是一个异步 trait，定义了获取和释放锁的方法。具体方法包括：
   - lock(&self)：获取锁，如果锁已经被持有，将会等待直到锁被释放。
   - try_lock(&self)：尝试获取锁，如果锁当前被持有，则立即返回 None，否则返回一个拥有锁的实例。
   - unlock(&self)：释放锁。

2. OwnedLock<T> trait：继承 Lock<T> trait，添加了拥有锁的方法。这个 trait 可以被实现者声明为 unsafe。
   - owned_lock(self)：获取拥有锁的实例。

3. AwaitLock<T> trait：继承 Lock<T> trait，添加了等待锁的方法。这个 trait 可以被实现者声明为 unsafe。
   - poll_lock(&self, cx: &mut Context) -> Poll<Option<Self::Guard>>：轮询地获取锁的权，如果锁当前被持有，则返回 `Poll::Pending` 状态；如果可以获取锁，返回一个拥有锁的实例。

这些 trait 的目的是为了实现 Tokio 调度器中的锁机制，提供可靠的异步锁操作，以确保并发任务的正确执行顺序和资源互斥。同时，通过拥有锁和等待锁的方法，提供了更灵活的用法，以满足不同的需求。

