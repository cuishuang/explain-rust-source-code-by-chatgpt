# File: rust-clippy/clippy_utils/src/attrs.rs

在rust-clippy的源代码中，`clippy_utils/src/attrs.rs`文件的作用是处理Rust代码中的属性（attributes）。该文件提供了一些结构体和枚举，来帮助识别和解析各种属性。

具体来说，`LimitStack`结构体用于表示属性堆栈的限制。属性堆栈是一个用于保存和跟踪属性的列表，允许在检查Rust代码时创建和操作属性。`LimitStack`结构体内部维护一个`Vec<String>`，用于保存属性的名称。通过`LimitStack::check_push`方法，可以向属性堆栈中添加属性，并检查是否已达到属性堆栈的最大限制。

`DeprecationStatus`枚举用于表示特定属性的废弃状态。该枚举定义了以下几个变体：

- `Active`：表示属性是活动状态，并不处于废弃状态。
- `Deprecated`：表示属性已被废弃，但仍可使用。
- `Removed`：表示属性已被移除，不再可用。

`DeprecationStatus`枚举的目的是在检查属性时，能够区分不同废弃状态的属性，并根据需要进行相应的处理。

这些结构体和枚举在`clippy_utils/src/attrs.rs`文件中的定义和实现，将属性的相关操作和状态进行了封装和简化，使得rust-clippy能够更方便地解析和处理Rust代码中的属性，并提供相应的警告和建议。

