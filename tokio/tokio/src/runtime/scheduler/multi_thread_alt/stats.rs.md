# File: tokio/tokio/src/runtime/scheduler/multi_thread_alt/stats.rs

在tokio的源代码中，`stats.rs`文件是位于`tokio/src/runtime/scheduler/multi_thread_alt/`目录下的一个文件。它的作用是提供了一些用于统计和记录调度器（scheduler）性能指标的数据结构和方法。

首先让我们来了解一下`Stats` struct。这个struct用于跟踪调度器的运行时统计信息，包括线程池（threadpool）的数量、当前线程的任务数目等。`Stats` struct的定义如下：

```rust
pub(crate) struct Stats {
    capacity: usize,
    threads: usize,
    idle: AtomicUsize,
    metrics: Metrics,
    counter: Mutex<Counter>,
    time: Mutex<Time>,
}
```

其中，`capacity`表示线程池的容量，`threads`表示当前线程池的线程数目，`idle`是一个`AtomicUsize`类型的原子计数器，用于表示当前处于空闲状态的线程数目。`metrics`是一个`Metrics`类型的实例，用于存储其他的性能指标。`counter`和`time`是两个用于计算平均时间和计数的辅助结构。

接下来是`Ephemeral` struct。这个struct是一个记录在调度过程中一次具体调度事件的数据结构。它用于在切换到其他任务之前记录上一个任务的运行时数据，以便统计和分析调度器的性能。`Ephemeral` struct的定义如下：

```rust
struct Ephemeral<'a> {
    time: Instant,
    prev: &'a mut Counter,
    eta: usize,
}
```

其中，`time`是记录任务开始执行的时间的`Instant`类型的实例。`prev`是一个`&'a mut Counter`类型的可变引用，用于记录上一个任务完成的时间。`eta`表示当前任务还需要的估计时间。

在`Ephemeral` struct中，还有一个关键的方法`started`，它被用于更新计数器和计算任务执行时间。这个方法的定义如下：

```rust
fn started(&mut self) {
    let now = Instant::now();
    let elapsed = now - self.time;

    // 更新计数器
    self.prev.started(elapsed);
    // 更新下一个任务的估计时间
    self.eta = self.prev.ix;
    // 更新当前任务的开始时间
    self.time = now;
}
```

这样，`Ephemeral`结构体通过`started`方法可以在每个任务开始时通过记录时间来计算任务执行的耗时。

综上所述，`stats.rs`文件中的`Stats`和`Ephemeral`结构体分别用于跟踪调度器的运行时统计信息和记录调度过程中的运行时数据，从而实现性能指标的统计和分析。

