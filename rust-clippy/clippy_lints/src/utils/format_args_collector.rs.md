# File: rust-clippy/clippy_lints/src/utils/format_args_collector.rs

在rust-clippy的源代码中，`format_args_collector.rs`文件的作用是处理`format_args!`宏调用中的参数，以便进一步检查和分析。

在`format_args_collector.rs`文件中，存在多个`FormatArgsCollector`结构体，其作用如下：

1. `FormatArgsCollector`: 这个结构体是最顶层的`FormatArgsCollector`，用于收集`format_args!`宏的参数，并提供一个公共方法`collect`来获取收集的参数。
2. `ImmediateCollector`: 这个结构体用于直接收集参数，即`format_args!("{:?}", foo)`中的`{:?}`，它实现了`CollectArgs` trait，并提供一个公共方法`collect`来获取收集的参数。
3. `MostlyConstantCollector`: 这个结构体用于收集大部分是常量的参数，也就是不包含格式化字符串占位符的参数，例如`format_args!("Hello, world!")`，它实现了`CollectArgs` trait，并提供一个公共方法`collect`来获取收集的参数。
4. `TypeCheckCollector`: 这个结构体用于进行类型检查，例如检查参数类型是否与格式化字符串一致，它实现了`CollectArgs` trait，并提供一个公共方法`collect`来获取收集的参数。

这些`FormatArgsCollector`结构体的目的是为了收集`format_args!`宏中的参数，并在静态分析和代码检查过程中提供所需的信息。它们通过实现`CollectArgs` trait来处理不同类型的参数，并通过提供的公共方法来获取收集的结果。这样可以方便地对参数进行各种检查和分析，以确保代码的正确性和安全性。

