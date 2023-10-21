# File: cargo/src/cargo/ops/cargo_output_metadata.rs

cargo_output_metadata.rs文件的作用是处理Cargo命令`cargo metadata`的输出。

文件中定义了几个重要的结构体：

1. OutputMetadataOptions：存储了执行`cargo metadata`命令时的参数和配置选项，例如包含的工作目录、输出格式等。

2. ExportInfo：表示一个导出的项，包含了导出项的名称、路径、类型等信息。

3. MetadataResolve：表示`cargo metadata`命令的解析结果，包含了整个项目的元数据信息，例如包、依赖关系、workspace等。

4. MetadataResolveNode：表示一个依赖项，包含了依赖项的名称、版本、类型等信息。

5. Dep：表示一个依赖项，包含了依赖项的名称、版本、路径等信息。

6. DepKindInfo：表示编译目标类型信息，表示了一个crate可以作为哪些不同类型的依赖项（例如编译器、库、可执行文件等）。

在代码中，`OutputMetadataOptions`结构体用于存储命令行参数和配置选项，`MetadataResolve`用于存储所有项目的元数据，`MetadataResolveNode`和`Dep`用于描述依赖关系，`DepKindInfo`用于描述编译目标类型信息。

这些结构体的目的是为了解析和存储`cargo metadata`命令的输出，并提供相应的 API 接口给其他模块使用，以方便用户和其他工具获取和处理项目的元数据信息。

