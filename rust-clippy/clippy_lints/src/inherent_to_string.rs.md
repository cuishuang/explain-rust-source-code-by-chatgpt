# File: rust-clippy/clippy_lints/src/inherent_to_string.rs

文件 `inherent_to_string.rs` 是 `rust-clippy` 中的一个特定 lint，用于检查是否有自动实现的 `ToString` trait，而不是实现 `Display` trait。它帮助开发者遵循 Rust 社区提倡的最佳实践，以更加明确和可控的方式实现类型的输出行为。

`rust-clippy` 是一个 Rust 代码检查工具，用于帮助开发者发现潜在的代码问题和提高代码质量。它提供了各种 lint（警告）来检查代码中的不良模式、潜在错误和可优化的代码，帮助开发者写出更可靠和高效的代码。

在 Rust 中，`ToString` 和 `Display` 是两个用于类型转换和显示格式化代码的 trait。

`ToString` trait 是 Rust 内置的 trait，用于将一个类型转换为字符串。它定义了一个 `to_string` 方法，该方法将该类型的实例转换为字符串表示形式。一般来说，如果希望将类型转换为字符串并打印出来，可以使用 `to_string` 方法。

`Display` trait 也是 Rust 内置的 trait，用于控制类型的显示格式。它定义了一个 `fmt` 方法，可以通过格式化字符串来指定类型的显示方式。通常在需要自定义类型如何被展示的时候，会实现 `Display` trait，并在 `fmt` 方法中定义输出格式。

这个 lint 的目的是鼓励开发者实现 `Display` 而不是 `ToString`，因为实现 `Display` trait 可以给类型的输出行为提供更多灵活性和控制权。`Display` trait 允许开发者自定义类型的展示方式，使其更符合语义和需求。相比之下，自动实现 `ToString` 通常使用默认的格式，可能无法提供足够定制化的输出。

因此，`inherent_to_string.rs` 文件中的 lint 的作用是向开发者发出警告，提醒他们优先实现 `Display` trait 而不是依赖自动实现的 `ToString` trait，以获得更好的代码质量和可维护性。它通过静态分析检查代码中的类型转换，并对发现的潜在问题发出警告。

