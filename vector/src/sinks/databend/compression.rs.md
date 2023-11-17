# File: vector/src/sinks/databend/compression.rs

在Rust生态中，vector项目是一个开源的数据处理工具，主要用于实时数据的采集、转换和传输。在vector项目中，vector/src/sinks/databend/compression.rs文件是用于提供数据压缩功能的组件。

该文件定义了一个名为`DatabendCompression`的枚举类型，用于选择不同的数据压缩算法和配置。该枚举类型有以下几个成员：

1. `Uncompressed`：表示不使用数据压缩，可以直接将数据写入输出。
2. `Lz4 { level: i32 }`：表示使用LZ4算法对数据进行压缩。`level`参数指定了压缩级别，值越大表示压缩程度越高但速度越慢。
3. `Snappy`：表示使用Snappy算法对数据进行压缩，这是一种快速但压缩比较低的算法。
4. `Zstd { level: i32 }`：表示使用Zstd算法对数据进行压缩。`level`参数指定了压缩级别，值越大表示压缩程度越高但速度越慢。

`DatabendCompression`枚举类型的作用是让用户可以根据需求选择适合的数据压缩算法，以减小数据的存储空间或传输带宽。在vector项目的源代码中，可以通过使用`DatabendCompression`枚举类型的成员来配置数据压缩功能。

