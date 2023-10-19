# File: tokio/tokio/src/sync/mutex.rs

在tokio的源代码中，`tokio/tokio/src/sync/mutex.rs`这个文件定义了`Mutex`和相关的几个结构体，提供了对互斥锁的支持。

`Mutex<T>`是一个原子的可变共享状态，通过使用互斥锁来保证线程之间对共享数据的互斥访问。它提供了`lock()`方法用于获取互斥锁的所有权，并返回一个`MutexGuard`类型的锁保护结构体，它实现了`Deref`和`DerefMut`，可以用来直接访问被锁保护的数据。当`MutexGuard`离开作用域时，它会自动释放互斥锁。需要注意的是，只有当前线程持有锁时才能对共享数据进行修改，其他线程需要等待锁被释放。

`MutexGuard`是一个锁保护结构体，实现了`Drop`特性，在其离开作用域时自动释放锁。它还实现了`Deref`和`DerefMut`特性，使得可以像访问数据一样访问锁保护的共享数据。`MutexGuard`还有一个重要的特性，当当前线程持有锁时，它是不可跨线程传递的。这是为了防止多线程之间共享指向相同数据的引用，在数据竞争的情况下导致意料之外的行为。

`OwnedMutexGuard`是一个拥有所有权的锁保护结构体，它可以在不同线程之间传递。它的主要作用是在`async`函数中使用`Mutex`，以便将互斥锁的所有权在不同的任务之间传递。

`MappedMutexGuard`是一个映射锁保护结构体，它在获取互斥锁的同时通过选取其中的一部分数据进行映射访问。这样可以避免对整个数据结构加锁，提高并发性能。

`OwnedMappedMutexGuard`是一个拥有所有权的映射锁保护结构体，可以在不同线程之间传递。

`MutexGuardInner`、`OwnedMutexGuardInner`、`MappedMutexGuardInner`和`OwnedMappedMutexGuardInner`是内部结构体，用于实现锁保护和所有权转移的底层逻辑。

最后，`TryLockError`是一个枚举类型，表示尝试获取互斥锁失败的情况，它有两个变体：`WouldBlock`表示当前无法获取锁，因为它正被另一个线程持有，`Poisoned`表示在获取锁的过程中发生了错误。

