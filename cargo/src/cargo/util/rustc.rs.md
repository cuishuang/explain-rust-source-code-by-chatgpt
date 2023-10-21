# File: cargo/src/cargo/util/rustc.rs

cargo/src/cargo/util/rustc.rs 文件在 Rust Cargo 中的作用是与 Rust 编译器（Rustc）相关的工具函数和数据结构的定义。

1. Rustc 结构体：Rustc 结构体定义了与 Rustc 相关的属性和行为。它包含一些特定于 Rustc 的信息，如二进制路径和版本，以及与编译器交互的方法。Rustc 结构体的方法允许通过命令行参数调用 Rustc，并收集和处理 Rustc 输出。

2. Cache 结构体：Cache 结构体定义了用于缓存 Rustc 编译输出的数据结构和方法。它包含一个哈希表，用于将输入参数与编译后的输出进行关联。Cache 结构体的方法用于从缓存中获取编译输出，或将新的编译输出存储到缓存中。

3. CacheData 结构体：CacheData 结构体定义了缓存中存储的编译输出的数据结构。它包含编译输出的路径、依赖关系图和编译后的二进制文件的哈希值等信息。CacheData 结构体的方法用于加载和保存缓存数据。

4. Output 结构体：Output 结构体定义了 Rustc 编译输出的数据结构。它包含编译产生的目标文件、二进制文件和其他输出文件的路径，以及编译错误和警告的信息。Output 结构体的方法用于从 Rustc 输出中解析这些信息。

总结：cargo/src/cargo/util/rustc.rs 文件定义了与 Rustc 相关的工具函数和数据结构，包括 Rustc 结构体、Cache 结构体、CacheData 结构体和 Output 结构体。这些数据结构和方法用于与 Rustc 进行交互、缓存编译输出，并解析编译输出的信息，以实现 Rust 项目的构建与编译。

