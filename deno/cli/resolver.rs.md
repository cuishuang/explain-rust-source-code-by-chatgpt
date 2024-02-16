# File: /Users/fliter/rust-contribute/deno/cli/resolver.rs

在Deno项目的源代码中，/Users/fliter/rust-contribute/deno/cli/resolver.rs这个文件的作用是实现了模块解析器（resolver）的功能。模块解析器在Deno中负责解析和加载JavaScript模块文件。

具体地，这个文件定义了如下一些结构体（struct）和枚举类型（enum）：

1. ModuleCodeStringSource：这个结构体表示模块代码字符串的来源。它包含一个字符串字段，用于存储模块的代码。

2. CliNodeResolver：这个结构体实现了模块解析器的逻辑。它包含一个方法（resolve），通过解析和加载模块的依赖关系，最终获取到所有相关模块的代码。

3. NpmModuleLoader：这个结构体是CliNodeResolver的一部分，负责加载和解析NPM模块。

4. CjsResolutionStore：这个结构体是CliNodeResolver的一部分，是一个互斥锁（Mutex）的HashSet，用于存储已解析的CommonJS模块。

5. MappedSpecifierResolver：这个结构体负责解析和处理通过@deno标记的特殊模块规范。

6. CliGraphResolver：这个结构体是CliNodeResolver的一部分，负责管理模块依赖图的解析和加载。

7. CliGraphResolverOptions：这个结构体是CliGraphResolver的配置选项。

8. SloppyImportsStatCache：这个枚举表示模块的状态缓存。

9. SloppyImportsResolver：这个枚举表示为了解析Sloppy Imports而添加的解析器。

此外，还有一些枚举类型：

1. MappedResolution：这个枚举表示模块的映射解析状态。

2. SloppyImportsFsEntry：这个枚举表示Sloppy Imports的文件系统入口。

3. SloppyImportsResolution：这个枚举表示Sloppy Imports的解析状态。

这些结构体和枚举类型一起协同工作，构成了Deno模块解析器的核心部分，负责解析和加载模块的依赖关系，以及处理Sloppy Imports和特殊模块规范等功能。

