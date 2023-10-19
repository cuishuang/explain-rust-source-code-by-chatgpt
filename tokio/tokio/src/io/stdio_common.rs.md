# File: tokio/tokio/src/io/stdio_common.rs

在Tokio源代码中，stdio_common.rs文件的作用是提供了与标准输入和标准输出相关的功能。

该文件定义了一些 struct，包括 SplitByUtf8BoundaryIfWindows<W>、TextMockWriter 和 LoggingMockWriter。

1. SplitByUtf8BoundaryIfWindows<W>: 这是一个用于处理 Windows 平台的特定结构体。Windows平台下，读取标准输入时可能会读取到不完整的 UTF-8 字符，因为标准输入输出在 Windows 上以字节流的形式提供。SplitByUtf8BoundaryIfWindows<W> 的作用是确保读取的数据按照正确的 UTF-8 字符边界分割。

2. TextMockWriter: 这是一个用于模拟标准输出的结构体。它实现了 Write trait，可以将输出数据保存在内存中，而不是实际写入到标准输出。

3. LoggingMockWriter: 这是另一个用于模拟标准输出的结构体。它实现了 Write trait，但它不仅将输出保存在内存中，还将其记录下来以进行日志记录。这在调试和测试过程中非常有用，可以将输出用于后续分析和验证。

总之，stdio_common.rs 文件中的这些 struct 提供了处理标准输入输出的功能，包括跨平台的 UTF-8 字符边界处理和模拟标准输出。

