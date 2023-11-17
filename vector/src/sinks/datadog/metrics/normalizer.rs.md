# File: vector/src/sinks/datadog/metrics/normalizer.rs

在Rust生态的Vector项目中，vector/src/sinks/datadog/metrics/normalizer.rs文件的作用是为Datadog监控服务的指标数据提供规范化转换。该文件中定义了一个叫做DatadogMetricsNormalizer的结构体，它是用来处理指标数据的规范化的核心组件。

DatadogMetricsNormalizer结构体的作用是将各种不同格式的指标数据转换为Datadog监控服务所需的规范格式。它包含了一些方法和逻辑，用于对指标数据的标签、名称、值等进行转换和处理。

该结构体中包含了多个其他的结构体，以实现不同的功能。这些结构体包括：

1. DimensionNormalizer: 用于对指标数据的标签进行规范化转换。它根据Datadog监控服务的要求，将标签转换为特定的格式，并且按照一定的规则对标签进行排序和过滤。

2. ValueNormalizer: 用于对指标数据的值进行规范化转换。它将数据的值转换为浮点数，并且处理数据的单位和精度。

3. MetricTypeNormalizer: 用于对指标数据的名称进行规范化转换。它根据Datadog监控服务的要求，对指标名称进行处理，并将其转换为特定的格式。

这些结构体共同协作，使得DatadogMetricsNormalizer能够将各种格式的指标数据转换为符合Datadog监控服务的规范。通过对指标数据进行规范化，可以确保数据的一致性和可靠性，使得数据能够被准确地存储和使用。

