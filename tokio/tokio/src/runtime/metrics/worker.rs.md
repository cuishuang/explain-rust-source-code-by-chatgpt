# File: tokio/tokio/src/runtime/metrics/worker.rs

文件目录：tokio/tokio/src/runtime/metrics/worker.rs

作用：这个文件是tokio（一个基于 Rust 的异步编程框架）运行时的度量指标模块的一部分，负责收集和记录与 Worker 相关的度量指标。

文件详细介绍：

1. `pub(crate) struct WorkerMetrics`: WorkerMetrics 结构体是 Worker 的度量指标收集器，负责记录与工作线程（worker）相关的度量指标。它包含了以下字段：

   - `task_count`: 记录当前工作线程正在执行的任务数量。
   - `idle_start_time`: 记录工作线程进入空闲状态的起始时间。
   - `idle_time`: 记录工作线程的总空闲时间。
   - `work_start_time`: 记录工作线程开始执行任务的起始时间。
   - `work_time`: 记录工作线程的总执行任务时间。
   - `poll_count`: 记录工作线程调用 `Poll::poll` 的次数。

2. `impl WorkerMetrics`: WorkerMetrics 结构体的实现部分定义了与度量指标相关的方法：

   - `new() -> Self`: 创建一个新的 WorkerMetrics 实例。
   - `on_work_start(&mut self)`: 在工作线程开始执行任务时更新度量指标。
   - `on_work_end(&mut self)`: 在工作线程完成任务时更新度量指标。
   - `on_poll(&mut self)`: 在工作线程调用 `Poll::poll` 时更新度量指标。
   - `on_idle_start(&mut self)`: 在工作线程进入空闲状态时更新度量指标。
   - `on_idle_end(&mut self)`: 在工作线程结束空闲状态时更新度量指标。

3. `pub(crate) struct WorkerMetricsRecorder`: WorkerMetricsRecorder 结构体是度量指标的记录器，负责将 WorkerMetrics 中的度量指标记录到度量输出源（MetricSink）中。

   - `record_metrics(&self, metrics: &WorkerMetrics)`: 将 Worker 的度量指标记录到度量输出源中。

这个文件的作用是通过 WorkerMetrics 结构体和 WorkerMetricsRecorder 结构体实现了度量指标的收集和记录功能，用于统计和监测 tokio 运行时中工作线程的执行情况，包括任务数量、空闲时间、任务执行时间等。这些度量指标的记录对于分析、调优和监控 tokio 运行时的性能和健康状态非常有帮助。

