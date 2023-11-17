# File: vector/src/api/schema/metrics/host.rs

在Rust生态vector项目的源代码中，`host.rs`文件是用于定义主机相关的度量指标的模块。

具体而言，该文件包含了以下几个结构体：

1. `MemoryMetrics(Vec<Metric>)`: 表示内存指标的结构体。它持有一个`Metric`类型的向量，表示内存相关的度量指标，比如总内存使用量、可用内存等。

2. `SwapMetrics(Vec<Metric>)`: 表示交换空间指标的结构体。它持有一个`Metric`类型的向量，表示交换空间相关的度量指标，比如交换空间使用量、空闲交换空间等。

3. `CpuMetrics(Vec<Metric>)`: 表示CPU指标的结构体。它持有一个`Metric`类型的向量，表示CPU相关的度量指标，比如CPU使用率、CPU温度等。

4. `LoadAverageMetrics(Vec<Metric>)`: 表示负载平均值指标的结构体。它持有一个`Metric`类型的向量，表示主机的负载平均值指标，比如1分钟、5分钟和15分钟负载平均值。

5. `NetworkMetrics(Vec<Metric>)`: 表示网络指标的结构体。它持有一个`Metric`类型的向量，表示网络相关的度量指标，比如接收和发送的数据量、接收和发送的数据包数等。

6. `FileSystemMetrics(Vec<Metric>)`: 表示文件系统指标的结构体。它持有一个`Metric`类型的向量，表示文件系统相关的度量指标，比如文件系统使用量、文件系统剩余量等。

7. `DiskMetrics(Vec<Metric>)`: 表示磁盘指标的结构体。它持有一个`Metric`类型的向量，表示磁盘相关的度量指标，比如磁盘读写速率、磁盘使用率等。

8. `HostMetrics(host_metrics::HostMetrics)`: 表示主机指标集合的结构体。它持有一个`host_metrics::HostMetrics`类型的对象，用于组织上述各种指标。

这些结构体的作用是提供一个统一的数据结构，用于存储和传递主机相关的度量指标。每个结构体都包含一个向量，该向量存储了具体的度量指标，并根据需要提供了相应的方法来操作和访问这些指标。通过这些结构体，开发者可以方便地获取和处理主机的各项度量指标数据。

