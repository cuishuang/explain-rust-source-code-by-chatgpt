# File: cargo/src/bin/cargo/commands/clean.rs

cargo/src/bin/cargo/commands/clean.rs 这个文件是 Rust Cargo 工具中的一个命令文件，它实现了 `cargo clean` 命令的功能。`cargo clean` 命令用于清理当前项目的构建输出和临时文件。

在 `clean.rs` 文件中，主要完成以下几个任务：

1. 获取命令行参数和配置：首先，该文件会调用 `App` 结构体创建命令行应用程序，定义了 `clean` 命令的名称、版本、用法和描述等信息。然后，通过 `args` 函数解析命令行参数，并读取配置文件中的相关配置。

2. 构建清理目标列表：接下来，`main` 函数通过调用 `clean` 函数来构建清理目标列表。`clean` 函数会遍历当前项目中的所有包，并根据其类型和配置，构建需要清理的目标列表。清理目标主要包括项目的构建目录、生成的二进制可执行文件、生成的库文件和临时文件等。

3. 执行清理操作：最后，`main` 函数会遍历清理目标列表，并逐个执行清理操作。清理操作主要包括删除文件和目录，调用系统命令来清理构建产物。

总体来说，`clean.rs` 文件通过定义 `cargo clean` 命令的行为和逻辑，实现了清理当前项目构建输出和临时文件的功能。通过调用系统命令和删除文件操作，清理产物可以帮助用户节省磁盘空间，并确保项目在下一次构建时从头开始。

