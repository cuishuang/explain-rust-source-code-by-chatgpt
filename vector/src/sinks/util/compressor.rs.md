# File: vector/src/sinks/util/compressor.rs

在Rust生态vector项目中，vector/src/sinks/util/compressor.rs文件的作用是提供了压缩数据的功能。该文件实现了一个Compressor结构体和一个Writer枚举。

Compressor结构体有三个字段：
1. `compression`: 表示压缩算法的枚举类型。它指定了压缩数据使用的算法，比如gzip、lz4等。
2. `buffer_size`: 表示压缩时使用的缓冲区大小。数据会被分批压缩，这个字段决定了每个批次的大小。
3. `compressor`: 使用具体压缩算法实现的结构体。根据`compression`字段的不同，可以有不同的底层压缩算法实现。

Writer枚举定义了不同的编码写入器类型：
1. `FileStreamWriter`: 将压缩数据写入到文件。
2. `GzipWriter`: 将压缩数据写入到gzip格式的文件。
3. `Lz4Writer`: 将压缩数据写入到lz4格式的文件。
4. `NoopWriter`: 不进行压缩，直接将数据写入原始文件。

Compressor结构体的主要作用是提供了压缩数据的接口和内部状态管理，它可以接收输入数据并使用底层压缩算法进行压缩，并将压缩后的数据写入到目标Writer中。

Writer枚举的作用是提供了不同类型的输出方式，可以将压缩后的数据写入到不同格式的文件或者直接写入原始文件中，根据用户的需求选择不同的输出方式。

总之，Compressor结构体和Writer枚举的组合提供了向不同输出目标写入压缩数据的功能，并且支持不同的压缩算法。这使得我们可以根据需求定制化地选择合适的压缩方式和输出目标。

