# File: vector/lib/file-source/src/lib.rs

vector/lib/file-source/src/lib.rs文件是Vector项目中文件源的实现，它定义了与文件相关的数据源的结构和功能。

该文件主要包含了两个enum：ReadFrom和ReadFromConfig。

ReadFrom是一个枚举类型，用于指定文件源的读取方式。它包含了以下四个变体：
- Stdin：从标准输入流读取数据。
- File：从文件读取数据。
- Glob：通过通配符匹配多个文件，并从这些文件中读取数据。
- Tail：从文件的末尾开始读取数据，实现文件的实时监控。

ReadFromConfig是一个枚举类型，用于配置文件源的参数。它包含了以下四个变体：
- StdinConfig：用于配置从标准输入流读取数据的参数。
- FileConfig：用于配置从文件读取数据的参数。
- GlobConfig：用于配置通过通配符匹配多个文件并读取数据的参数。
- TailConfig：用于配置从文件末尾开始读取实时数据的参数。

这些枚举类型提供了对文件源的不同读取方式和配置参数的抽象，可以根据具体需求选择不同的变体。通过使用这些枚举类型以及相关的实现方法，可以方便地在项目中实现文件源的读取和配置功能。

