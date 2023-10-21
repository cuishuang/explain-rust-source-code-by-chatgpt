# File: cargo/crates/mdman/src/format/text.rs

在Rust Cargo的源代码中，cargo/crates/mdman/src/format/text.rs文件是用于定义文本格式的文件。它包含了TextFormatter、TextRenderer和Table三个结构体。

TextFormatter结构体是一个用于格式化文本的工具，它实现了从AST（抽象语法树）生成文本的功能。它将AST中的标记转换为相应的文本格式，并处理了行的缩进、对齐等格式要求。通过TextFormatter，可以将AST转换成可读性强的文本输出。

TextRenderer是一个文本渲染器，它负责将格式化后的文本输出到指定的输出设备，例如终端或文件。TextRenderer中包含了一个TextFormatter实例，并提供了一些方法来渲染和显示文本。

Table结构体是一个用于呈现表格形式的结构。它实现了在终端上输出格式化的表格，并提供了一些方法来定义表头、列宽、填充内容等。通过Table，可以将数据以表格形式展示出来，使其更易读和理解。

这些结构体共同协作，用于将AST表示的Markdown文档转换为可读的文本输出，并提供了适当的格式化和渲染功能，以便用户可以在终端或其他输出设备上正确显示文本。

