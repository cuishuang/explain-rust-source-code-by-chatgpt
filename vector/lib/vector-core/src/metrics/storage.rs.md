# File: vector/lib/vector-core/src/metrics/storage.rs

在Rust生态的Vector项目中，`vector/lib/vector-core/src/metrics/storage.rs`文件的作用是定义了用于存储度量指标数据的数据结构和相关的方法。

该文件中定义了三个重要的结构体：`VectorStorage`、`AtomicF64`和`Histogram`。

- `VectorStorage`结构体是用于存储度量指标数据的主要数据结构。它包含了一个哈希表（`HashMap`），用于存储不同指标的名称和对应的指标数据。`VectorStorage`还提供了一组方法，用于对指标数据进行增加、获取和重置等操作。

- `AtomicF64`结构体是一个原子浮点数类型，用于在并发环境下进行原子操作，保证数据的线程安全性。它实现了`std::sync::atomic::AtomicU64`类型并提供了一组方法来对浮点数进行原子操作。

- `Histogram`结构体是用于表示指标数据的分布情况的统计工具。它可以接收一系列浮点数样本，并根据这些样本数据构建一个直方图。`Histogram`提供了一组方法，用于计算直方图的各种统计量，如均值、中位数、最大值、最小值等。

这些结构体在Vector项目中用于存储和处理度量指标数据，提供了线程安全性和统计分析的功能，有助于开发者对应用的性能进行监控和优化。

