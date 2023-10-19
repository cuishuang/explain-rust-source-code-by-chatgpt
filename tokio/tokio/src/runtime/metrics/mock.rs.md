# File: tokio/tokio/src/runtime/metrics/mock.rs

tokio/tokio/src/runtime/metrics/mock.rs 这个文件定义了模拟器(metrics/mock.rs)的结构和方法，用于测试和模拟Tokio的度量指标。它主要用于在测试环境中使用假的度量指标，以验证调度器和工作器的行为是否正确。

下面是每个结构的作用：

1. SchedulerMetrics: 这个结构用于模拟调度器的度量指标。它包含了对调度程序运行次数、挂起次数、任务执行次数等指标的跟踪。同时，它还提供了一些方法用于增加相应的指标数，在测试中可以通过调用这些方法来模拟调度器的行为并验证其正确性。

2. WorkerMetrics: WorkerMetrics 用于模拟工作器的度量指标。它记录了工作者的运行次数、任务执行次数、等待队列的长度等指标。它也提供了类似SchedulerMetrics的增加指标的方法，供测试中使用。

3. MetricsBatch: MetricsBatch 结构用来存储度量指标的批处理数据。它是一个简单的结构体，定义了需要一次批量处理的度量指标和对应的值。主要用于模拟的度量器收集指标数据。

4. HistogramBuilder: HistogramBuilder 是用于构建直方图(Histogram)的辅助工具。直方图是一种经常用于度量、统计和分析数据分布的工具。它提供了写入数据和获取统计数据的方法，用于对任务执行时间的统计。

以上这些结构体主要用于在测试环境中模拟和记录度量指标的数据，以确保Tokio的调度器和工作器的行为符合预期。

