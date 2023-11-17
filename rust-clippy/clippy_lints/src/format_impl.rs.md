# File: rust-clippy/clippy_lints/src/format_impl.rs

在rust-clippy的源代码中，`format_impl.rs` 文件的作用是实现用于格式化代码的 lint 检查的相关功能。具体来说，它定义了 `FormatTraitNames` 和 `FormatImpl` 结构体，并实现了 `Method` 和 `Impl` 这两个 trait。

`FormatTraitNames` 结构体用于表示需要进行格式化检查的 trait 名称。它包含了一个名为 `names` 的 `HashMap`，其中的键是 trait 名称的字符串，值是一个布尔值，表示该 trait 是否需要进行格式化检查。

`FormatImpl` 结构体用于表示需要进行格式化检查的 impl 块。它包含了一个名为 `methods` 的 `HashMap`，其中的键是方法名的字符串，值是一个 `Method` 结构体，表示该方法的格式化检查情况。

`Method` 结构体用于表示一个需要进行格式化检查的方法。它包含了一个名为 `args` 的 `Vec`，其中的元素是一个布尔值，表示该方法对应的参数是否需要格式化检查。

`Method` 结构体还包含了一个名为 `format_string` 的可选字段，用于表示该方法中的字符串是否需要格式化检查。

`Impl` trait 是一个用于处理 impl 块的 trait。它定义了一个 `on_impl` 方法，用于处理 impl 块，并将 impl 块中的方法添加到 `FormatImpl` 结构体的 `methods` 字段中。

`Method` trait 是一个用于处理方法的 trait。它定义了一个 `on_method` 方法，用于处理方法，并将方法的参数和字符串格式化检查的结果添加到 `Method` 结构体中。

总而言之，`format_impl.rs` 文件中的 `FormatTraitNames`、`FormatImpl`、`Method`、`Impl` 这些结构体和 trait 提供了用于格式化检查的数据结构和相关功能的实现。

