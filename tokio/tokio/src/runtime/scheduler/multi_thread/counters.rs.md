# File: tokio/tokio/src/runtime/scheduler/multi_thread/counters.rs

在tokio的源代码中，`counters.rs`文件位于多线程调度器模块下的`multi_thread`文件夹中，其作用是用于统计和跟踪多线程调度器的各种指标和计数器。

该文件中定义了三个`struct`，分别是`Blocking`、`SpinWait`和`Worker`。

1. `Blocking`结构体用于记录在运行时线程阻塞的次数。每当一个线程在任务上被阻塞时，`Blocking`的计数器就会增加。这些阻塞事件的计数器被用于评估和优化多线程调度器的性能。

2. `SpinWait`结构体用于记录自旋等待操作的次数。当一个线程需要等待一个条件被满足时，它可以通过自旋等待的方式来等待，而不是真正地阻塞线程。`SpinWait`的计数器用于衡量自旋等待的频率，以及确定是否需要调整等待策略。

3. `Worker`结构体用于记录和跟踪工作线程的状态。每个工作线程都有一个对应的`Worker`实例。`Worker`结构体中的计数器包括`idle`、`parked`、`notified`和`yielding`。`idle`表示线程处于空闲状态的时间；`parked`表示线程处于等待状态的时间；`notified`表示线程被通知唤醒的次数；`yielding`表示线程主动放弃CPU的次数。

这些计数器和跟踪器被用于分析和优化tokio多线程调度器的性能，从而改进任务的调度和线程的管理。

