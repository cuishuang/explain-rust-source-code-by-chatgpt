# File: miri/cargo-miri/src/util.rs

在Rust的miri项目中，`miri/cargo-miri/src/util.rs`文件是用于定义一些与运行Mir功能相关的工具函数和结构体。

`CrateRunEnv`结构体是一个表示包运行环境的数据结构。它包含了一些运行Mir的必要信息，如Rust编译器、运行Mir的root文件和其它参数。`CrateRunEnv`有两个实现，分别是`HostRunEnv`和`TargetRunEnv`，分别用于主机环境和目标环境。

`CrateRunInfo`枚举类型是用于表示Mir执行的不同方式。它有以下几个成员：
- `CompileAndRun`：表示编译并运行Mir。
- `RunMir`：表示直接运行Mir。
- `Unchanged`：表示Mir没有改变。
- `Error`：表示存在错误。

`MiriCommand`枚举类型用于表示Miri工具的不同命令。它包含以下成员：
- `Check`：表示执行Mir检查。
- `Test`：表示运行Mir测试。
- `Run`：表示运行Mir程序。
- `TestParser`：表示测试mir-json-parser模块。
- `Translate`：表示翻译Mir。

这些枚举类型和结构体的定义使得Mir工具有了更好的封装和管理能力，方便了Mir的编译、运行和测试等操作。

