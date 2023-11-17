# File: vector/src/components/validation/runner/config.rs

在Rust生态的Vector项目中，vector/src/components/validation/runner/config.rs文件的作用是定义用于配置验证运行器的数据结构和方法。

该文件中定义了名为`Config`的结构体，用于表示验证运行器的配置。该结构体中包含了以下字段：

1. `exit_on_success`: 一个布尔值，表示在验证成功时是否立即退出。
2. `exit_on_failure`: 一个布尔值，表示在验证失败时是否立即退出。
3. `run_once`: 一个布尔值，表示是否只运行一次验证。
4. `no_duplicates`: 一个布尔值，表示是否检查验证结果中是否存在重复的结果。
5. `timeout_secs`: 一个可选的整数，表示验证运行的超时时间，以秒为单位。

除了结构体定义之外，该文件还实现了`Config`结构体的方法，包括：

1. `default()`: 一个关联函数，返回一个默认的配置实例。
2. `validate(&self, errors: &mut Errors)`: 一个方法，用于验证配置的有效性。它接受一个错误集合作为参数，并在配置无效时将相应的错误添加到集合中。

至于`TopologyBuilder`结构体，它们并不直接与`config.rs`文件有关。`TopologyBuilder`是Vector中用于构建数据流拓扑的结构体，用于连接各种组件（如输入源、处理器和输出目标）以构建完整的数据管道。`TopologyBuilder`结构体负责创建和管理组件及其之间的连接，从而实现数据的收集、处理和传递。它是Vector项目中的核心组件之一，并负责整个数据流的控制和配置。不同的`TopologyBuilder`结构体可能有不同的作用和功能，具体取决于其所创建的组件类型和配置参数。

