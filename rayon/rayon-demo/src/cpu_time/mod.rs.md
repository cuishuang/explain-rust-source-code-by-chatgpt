# File: rayon/rayon-demo/src/cpu_time/mod.rs

在Rust的rayon crate中，rayon-demo/src/cpu_time/mod.rs文件是用于测量CPU时间的模块。这个模块定义了一个名为`CpuMeasure`的结构体，该结构体用于测量代码在特定CPU时间上的执行时间。

`CpuMeasure`结构体有三个字段，分别是`time`、`total_time`和`inner`。这些字段的作用如下：

1. `time`字段用于保存当前线程的开始执行的时间点。
2. `total_time`字段用于保存从线程开始执行到目前为止的总时间。
3. `inner`字段是一个Option类型的变量，用于嵌套测量。在测量嵌套代码块的时间时，可以将先前的`CpuMeasure`结构体存储在`inner`字段中。

`CpuMeasure`结构体还实现了一些方法，用于在代码块开始和结束时进行时间测量和计算。具体来说，`CpuMeasure`的方法如下：

1. `new()`方法用于创建一个新的`CpuMeasure`对象，初始化时间和总时间字段。
2. `start()`方法用于在代码块的开始处记录当前时间作为起始时间。
3. `stop_and_read()`方法用于在代码块的结束处停止测量并返回执行时间。
4. `elapsed_time()`方法用于计算开始到结束的时间间隔并返回一个表示执行时间的`Duration`对象。
5. `accumulate_time()`方法用于累加总时间字段。
6. `reset()`方法用于重置时间和总时间字段。

通过使用`CpuMeasure`结构体，可以方便地测量代码块在CPU上的执行时间，帮助用户进行性能分析和优化。

