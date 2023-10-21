# File: cargo/src/cargo/core/summary.rs

在Rust Cargo中，`cargo/src/cargo/core/summary.rs`文件的作用是定义了用于表示包（crate）摘要的结构和枚举。

`Summary`结构体是包的摘要信息，在Cargo中用于记录有关包的重要信息，例如包的名称、版本、目标名称、目标类型等等。`Summary`结构体包含以下字段：

- `package_id`: 表示包的唯一标识符
- `name`: 包的名称
- `version`: 包的版本
- `dependencies`: 表示包的依赖关系
- `features`: 表示包所支持的特性（features）
- `target`: 表示包的目标信息

`Inner`结构体在`Summary`结构体内部使用，用于存储摘要的具体信息，例如作者、文档、描述等等。

`FeatureValue`枚举是用于表示特性（features）的值，它有以下几个变体：

- `Bool`: 表示特性是一个布尔值（true/false）
- `Dep`: 表示特性是一个依赖项（dependency），指示依赖关系的名称和版本约束
- `String`: 表示特性是一个字符串
- `Table`: 表示特性是一个键值对表

使用`FeatureValue`枚举可以灵活地表示各种类型的特性值，这对于处理包的特性非常有用。

