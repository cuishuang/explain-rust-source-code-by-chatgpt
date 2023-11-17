# File: vector/src/sinks/statsd/normalizer.rs

在Rust生态的vector项目中，位于vector/src/sinks/statsd/normalizer.rs文件的StatsdNormalizer主要用于标准化StatsD指标名称和值。

StatsD是一种非常常用的指标收集和度量工具，它通过UDP协议接收度量数据，并将其发送到后端的度量汇总工具（如Graphite）或监控系统（如Datadog）。StatsD通常使用"metric.name:value|type"的格式来表示指标。

StatsdNormalizer模块内定义了三个结构体：

1. StatsdNormalizer：这是主要的标准化器结构体。它存储了一些配置信息和状态，并提供了方法来配置标准化规则、标准化指标名称和值。

2. NormalizerConfig：这个结构体用于存储标准化器的配置信息。配置信息主要包括匹配规则（matchers）和替换规则（replacers）。

   - 匹配规则：Matchers是一个正则表达式字符串的列表。它们用于匹配待标准化的指标名称或值。
   - 替换规则：Replacers是一个替换规则的列表，每个替换规则都包含一个匹配器和一个替换字符串。标准化器将使用替换规则来替换匹配到的指标名称或值。

3. NormalizedMetric：这是一个辅助结构体，用于存储已经被标准化的指标名称和值。

在StatsdNormalizer中，主要有以下几个关键方法：

1. new：用于创建并初始化一个新的StatsdNormalizer实例。它接收一个配置参数NormalizerConfig，并根据配置初始化标准化器。

2. normalize_metric：这是标准化器的核心方法之一。它接收一个Metric实例，将其名称和值进行标准化，并返回一个NormalizedMetric实例。

3. normalize_name/normalize_value：这两个方法分别用于标准化指标的名称和值。它们会根据配置的规则进行匹配和替换。

标准化器使用配置中的匹配规则遍历指标名称和值，并通过替换规则进行相应的替换。这样可以确保输入的指标名称和值符合指定的格式，并且与后端的度量汇总工具或监控系统兼容。

通过StatsdNormalizer，开发者可以轻松地配置和定制标准化规则，以满足不同的需求和场景。它提供了一种灵活和可扩展的方式来处理StatsD指标的标准化。

