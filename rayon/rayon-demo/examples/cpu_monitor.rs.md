# File: rayon/rayon-demo/examples/cpu_monitor.rs

在Rust rayon的源代码中，`rayon-demo/examples/cpu_monitor.rs`这个文件的作用是使用Rayon库来监控CPU的使用情况，并提供一个可视化的实时图表。

该文件使用了Rayon库的并行能力来计算一段时间内CPU的使用率，并将结果绘制成实时图表。通过这种方式，可以更直观地了解CPU的工作负载情况。

在文件中，定义了一个`Args`结构体，它用于解析命令行参数。`Args`结构体包含了几个字段：

1. `--num-steps`：指定监控的时间段内计算使用的步数，默认为1000000。
2. `--num-workers`：指定用于计算的线程数，默认为Rayon库根据CPU核心数自动选择的数目。
3. `--plot-interval`：指定图表刷新的时间间隔，默认为200毫秒。

这些参数可以通过命令行传入，用于定制化CPU监控的行为。

在`main`函数中，首先解析命令行参数，然后创建一个无限循环，每次循环中都使用Rayon库的`scope`函数来并行计算一定数量的步数。计算结束后，将CPU使用率加入一个全局的使用率列表。同时，定期将使用率列表的数据传递给绘图组件，并请求刷新图表。

为了确保线程安全，使用了`Arc<Mutex<Vec<(Instant, f32)>>`来共享使用率列表，以及`Sender<Vec<(Instant, f32)>>`和`Receiver<Vec<(Instant, f32)>>`来进行线程间通信。

整个文件的目的是提供一个简单实用的CPU监控工具，让开发者可以通过一个可视化的实时图表来观察CPU的负载。

