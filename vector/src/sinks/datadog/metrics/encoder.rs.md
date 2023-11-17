# File: vector/src/sinks/datadog/metrics/encoder.rs

文件`encoder.rs`的作用是实现将数据编码为Datadog格式的指标（metrics）。

`EncoderState`是一个枚举类型，表示编码器的状态。它有三个可能的取值：`Empty`表示编码器尚未开始编码数据，`Created`表示编码器已创建，并且已经开始编码数据，`Finished`表示编码器已完成数据编码。

`DatadogMetricsEncoder`是一个结构体，它包含用于编码Datadog指标的方法和数据。该结构体实现了一个数据编码器的基本功能，可以将数据转换为Datadog支持的格式。它包含一个`EncoderState`来表示编码器的状态，一个`Vec`来存储待编码的指标数据，以及其他与编码相关的参数和方法。

`CreateError`，`EncoderError`和`FinishError`分别是枚举类型，用于表示在编码过程中可能发生的错误。这些错误包括创建编码器错误、编码错误和完成编码错误。通过使用这些枚举类型，可以在出现错误时提供更具体的错误信息和错误处理方式。

总而言之，`encoder.rs`文件中定义了一组用于编码数据为Datadog指标的结构体和枚举类型。这些结构体和枚举类型提供了处理和报告编码过程中可能出现的错误的能力，并定义了数据编码器的基本功能。

