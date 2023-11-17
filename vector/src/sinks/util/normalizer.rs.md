# File: vector/src/sinks/util/normalizer.rs

在Rust生态的Vector项目中，vector/src/sinks/util/normalizer.rs这个文件是用于实现对日志事件进行规范化的功能。它包含一个名为Normalizer的公共结构体和几个相关的结构体来实现不同的规范化操作。

Normalizer<St>结构体是一个泛型结构体，用于对日志事件进行规范化处理。它接受一个状态类型St作为类型参数，可以在规范化过程中维护一些状态信息。

Normalizer结构体具有一些公共方法和字段，用于初始化、处理日志事件和输出规范化后的结果。

Normalizer结构体相关的其他结构体如下：

1. MetricNormalizer：用于规范化指标（Metric）类型的日志事件。它实现了Metric normalizer trait，并实现了抽象方法normalize_metric。

2. LogNormalizer：用于规范化日志（Log）类型的日志事件。它实现了Log normalizer trait，并实现了抽象方法normalize_log。

3. PayloadNormalizer：用于规范化载荷（Payload）类型的日志事件。它实现了Payload normalizer trait，并实现了抽象方法normalize_payload。

这些结构体共同的作用是将不同类型的日志事件进行规范化处理，确保它们具有一致的结构和格式。通过规范化，可以使得不同来源、不同格式的日志数据能够以统一的方式处理和分析，提高日志数据的可用性和可靠性。

