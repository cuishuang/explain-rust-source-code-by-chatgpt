# File: tokio/tokio/src/runtime/blocking/pool.rs

在tokio的源代码中，tokio/tokio/src/runtime/blocking/pool.rs文件是用于实现Blocking Pool的功能。Blocking Pool是一种用于处理阻塞式任务的线程池。

在文件中，有几个主要的结构体和枚举：

1. `BlockingPool`：这是Blocking Pool的主要结构体，用于管理和执行阻塞式任务。它实现了`Future` trait，并在其`poll`函数中处理阻塞任务的执行。

2. `Spawner`：这是一个用于在Blocking Pool中生成新任务的辅助结构体。它提供了一个`spawn_blocking`方法，该方法接收一个闭包作为参数，并将闭包封装成一个`Task`对象以在Blocking Pool中执行。

3. `SpawnerMetrics`：这个结构体用于跟踪Blocking Pool的指标。它包含了一些计数器，比如已完成的阻塞任务的数量。

4. `Inner`：该结构体用于在Blocking Pool中管理任务队列和线程池。它负责将任务添加到队列中，并在有空闲线程时将任务分发给线程。

5. `Shared`：这是一个包含了Blocking Pool所需共享状态的结构体。它包含了一个`Inner`结构体和一些用于跟踪线程池状态的变量。

6. `Task`：这是一个包含闭包的结构体，表示一个阻塞式任务。它实现了`Future` trait，并在其`poll`函数中执行闭包。

7. `Mandatory`：这是一个枚举，表示任务的优先级。它有两个可能的值：`Yes`和`No`。如果任务被标记为`Mandatory::Yes`，则它必须在当前线程中执行，而不是交给线程池。

8. `SpawnError`：这个枚举用于表示生成任务时可能出现的错误。它包含了一些可能的错误类型，例如线程池已满或生成任务过程中发生了错误。

总体来说，tokio/tokio/src/runtime/blocking/pool.rs文件中的这些结构体和枚举是用于实现并管理Blocking Pool的功能，包括在阻塞任务执行时的线程池管理，任务的生成和分发，以及跟踪任务执行情况和指标的跟踪。

