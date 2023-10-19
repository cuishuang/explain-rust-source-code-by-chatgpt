# File: tokio/tokio/src/sync/semaphore.rs

在tokio源代码中，tokio/tokio/src/sync/semaphore.rs文件的作用是实现信号量（semaphore）的功能。信号量是一种计数器，用于控制对共享资源的访问。

Semaphore结构体是一个简单的信号量实现，用于限制同时访问共享资源的线程数。它包含一个计数器和一个FIFO队列，用于存储等待许可（permit）的操作。

SemaphorePermit<'a>结构体是一个表示信号量许可的类型，它实现了Drop trait，在离开作用域时会自动释放许可。SemaphorePermit用于表示一个成功获取到的许可。

OwnedSemaphorePermit结构体也是一个表示信号量许可的类型，但它拥有自己的生命周期，不需要依赖作用域来释放许可。OwnedSemaphorePermit主要用于将许可的所有权传递给其他线程。

Semaphore结构体的主要方法包括：

- new()：创建一个新的信号量实例。
- acquire()：获取一个许可。如果当前没有可用的许可，则阻塞线程。
- try_acquire()：尝试获取一个许可，如果没有可用的许可，则立即返回。
- permits()：返回当前可用的许可数量。
- add_permits()：添加指定数量的许可到信号量。
- close()：关闭信号量，不再接受新的许可请求，并尝试取消等待中的所有请求。

SemaphorePermit<'a>的方法包括：

- downgrade()：将许可降级为OwnedSemaphorePermit，用于传递给其他线程。
- forget()：忘记这个许可，让许可自动释放。

OwnedSemaphorePermit的方法包括：

- permit()：从OwnedSemaphorePermit创建SemaphorePermit。
- forget()：忘记这个许可，让许可自动释放。

这些结构体和方法的组合使得在tokio中可以方便地实现对共享资源的并发访问控制和限制，从而实现高效且可靠的异步编程。

