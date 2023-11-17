# File: vector/src/utilization.rs

在Rust生态中，vector是一个用于处理、转换和传输数据的高性能数据处理管道。在vector的源代码中，vector/src/utilization.rs文件的作用是实现了一个用于计算资源利用率的模块。

Utilization<S>这个结构体是计算资源利用率的核心逻辑。它的泛型S表示状态，可以是任何可以用于计算资源利用率的状态类型。Utilization<S>结构体具有以下主要功能：

1. `new()`：用于创建一个新的Utilization实例。
2. `record()`：将一个资源使用记录传递给Utilization，以便其根据记录来计算和更新资源利用率。可以根据使用需求记录不同的资源，例如计算时间、内存或网络使用量等。
3. `interval()`: 设置资源利用率计算窗口的时间间隔。
4. `current_utilization()`：返回当前计算的资源利用率。

Timer结构体则用于记录资源的使用时间。它具有以下功能：

1. `new()`：创建一个新的Timer实例。
2. `start()`：开始计时器。
3. `stop()`：停止计时器，并返回经过的时间。

通过将Timer与Utilization结合使用，可以在pipeline的不同步骤中实时记录并计算各种资源的利用率，从而帮助分析和优化系统性能。

