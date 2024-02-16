# File: /Users/fliter/rust-contribute/deno/cli/tools/fmt.rs

在Deno项目的源代码中，/Users/fliter/rust-contribute/deno/cli/tools/fmt.rs文件主要负责实现Deno的代码格式化功能。代码格式化是指按照一定的规范和风格对代码文件进行重新排版，以提高代码的可读性和可维护性。

文件中定义了几个struct，其中比较重要的是FileContents结构体。FileContents结构体用于表示源代码文件内容，并提供了相关的方法和功能。主要包括以下几个作用：

1. 读取源代码文件内容：FileContents可以通过调用open方法读取指定文件路径下的源代码文件，并将文件内容存储在结构体中以供后续操作使用。

2. 解析源代码文件：FileContents提供了parse方法，可以将源代码文件内容解析为语法树（AST，Abstract Syntax Tree）。语法树是一种以树形结构组织的代码表达形式，便于进行代码分析和操作。

3. 格式化源代码：FileContents提供了format方法，用于对解析后的语法树进行格式化操作。格式化包括对代码的缩进、换行、空格等进行统一的规范化处理，使代码的风格更加一致。

FileContents结构体是Deno代码格式化的核心数据结构，通过它可以读取、解析和格式化源代码文件。除此之外，文件中还包含了一些辅助的函数和变量，用于处理格式化的具体细节和逻辑。

总的来说，/Users/fliter/rust-contribute/deno/cli/tools/fmt.rs文件中的FileContents结构体及相关代码，实现了Deno的代码格式化功能，通过读取、解析和格式化源代码文件，使代码符合统一的规范和风格要求。

