# File: tokio/tokio/src/process/unix/reap.rs

在Tokio源代码中，`reap.rs`文件的作用是处理子进程的回收。

具体来说，`reap.rs`定义了一个名为`Reaper`的结构体。`Reaper`是一个用于回收子进程的管理器，它监听由`ChildEvents`流产生的事件，并负责将这些事件转换为异步任务。它使用一个用于等待子进程退出的`epoll`实例，并根据子进程与关联的`W`类型（代表等待策略）进行错误处理。

`Reaper`结构体的定义如下：

```rust
pub struct Reaper<W> {
    wait: W,
    rx: mpsc::Receiver<PidEvent>,
    epoll: Result<Epoll<MioPoll>>,
    #[cfg(any(target_os = "linux", target_os = "android"))]
    epin4: Option<mux::Epoll>,
    #[cfg(any(target_os = "linux", target_os = "android"))]
    epin6: Option<mux::Epoll<UnsafeCell<MioPoll>>>,
}
```

其中，`W`是一个泛型参数，代表了等待策略。根据不同操作系统的特性，等待策略可以有不同的实现。

`Reaper`结构体中的字段有：
- `wait`：用于等待子进程退出的策略。
- `rx`：用于接收`ChildEvents`流中的事件的接收器。
- `epoll`：用于监听I/O事件的`epoll`实例。
- `epin4`和`epin6`：用于特定平台的I/O事件监听。

此外，`reap.rs`定义了`MockWait`和`MockStream`两个结构体。

`MockWait`结构体是一个用于模拟等待策略的实现，它不进行任何实际的等待，用于测试目的。

`MockStream`结构体是一个用于模拟I/O事件流的实现，它不依赖于真实的底层I/O机制，也用于测试目的。它实现了`Future`特性，因此可以与`await`表达式一起使用。

这些结构体的存在使得在测试或模拟环境下，能够方便地替代真实的等待策略或I/O事件流。

