# File: /Users/fliter/rust-contribute/deno/cli/module_loader.rs

在Deno项目的源代码中，文件`/Users/fliter/rust-contribute/deno/cli/module_loader.rs`是Deno的CLI模块加载器的实现。以下是每个struct的详细介绍：

1. `ModuleLoadPreparer`: 这个struct主要负责准备模块的加载。它会解析传入的模块文件，检查权限并设置模块的状态。

2. `PreparedModuleLoader`: 这个struct是ModuleLoadPreparer的实现，用于将模块准备工作委托给V8引擎完成。它实现了ModuleLoadPreparer trait，并为每个模块创建对应的资源。

3. `SharedCliModuleLoaderState`: 这个struct存储了CLI模块加载器的共享状态。它包含了已经准备好的模块加载器和源代码映射关系(即CliSourceMapGetter)。

4. `CliModuleLoaderFactory`: 这个struct是一个工厂，用于创建CLI模块加载器。它实现了ModuleLoaderFactory trait，并创建SharedCliModuleLoaderState。

5. `CliModuleLoader`: 这个struct是CLI模块加载器的实际实现。它实现了ModuleLoader trait，并用于加载和执行模块。它依赖于SharedCliModuleLoaderState，负责将模块委托给PreparedModuleLoader进行加载。

6. `CliSourceMapGetter`: 这个struct负责获取CLI模块的源代码映射关系。它实现了SourceMapGetter trait，并从SharedCliModuleLoaderState获取源代码映射数据。

通过这些struct的组合和调用，Deno的CLI模块加载器能够有效地加载和解析模块，并处理模块之间的依赖关系和执行顺序。

