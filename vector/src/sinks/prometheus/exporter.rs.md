# File: vector/src/sinks/prometheus/exporter.rs

在Rust生态vector项目中，vector/src/sinks/prometheus/exporter.rs文件的作用是实现了一个Prometheus导出器，用于将指标(metrics)导出到Prometheus。

下面是对每个struct的详细介绍：

1. PrometheusExporterConfig：这个struct用于配置Prometheus导出器的设置，包括绑定的IP地址、端口号等。

2. PrometheusExporter：这个struct是Prometheus导出器的主要实现，它包含了一个处理器(handler)，用于处理待导出的指标数据。它还提供了导出器的启动、停止和导出指标的方法。

3. MetricMetadata：这个struct保存了指标的元数据，包括指标的名称、标签等。

4. MetricRef：这个struct是对MetricMetadata的引用，它保存了指标数据和元数据的引用，用于传递给处理器进行导出。

5. PrometheusExporterMetricNormalizer：这个struct实现了MetricNormalizer trait，用于把待导出的指标数据转换成MetricRef。

6. Handler：这个trait定义了处理待导出指标的方法，具体的导出逻辑由实现Handler trait的struct来实现。

对于enum BuildError，它定义了在构建Prometheus导出器时可能出现的错误类型。具体的枚举值表示了不同的错误情况，比如缺失配置参数、参数错误等。这些错误类型可以帮助在构建过程中发现并处理潜在的问题。

