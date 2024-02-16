# File: /Users/fliter/rust-contribute/deno/cli/lsp/code_lens.rs

在Deno项目的源代码中，/Users/fliter/rust-contribute/deno/cli/lsp/code_lens.rs文件的作用是处理代码镜头（CodeLens）相关的逻辑。

首先，这个文件中定义了三个结构体：CodeLensData、DenoTestCollector和DocumentLinkSource。

1. CodeLensData: 这个结构体用来表示代码镜头数据，包含了一个位置(`range`)，表示代码片段的位置范围；一个命令(`command`)，表示关联的命令名称；以及一些扩展(`data`)，可以用来传递额外的信息。

2. DenoTestCollector: 这个结构体用来收集和管理Deno项目中的测试代码镜头。它会遍历源代码，识别测试函数，并生成对应的代码镜头数据。测试函数可以通过一定的命名约定自动识别。DenoTestCollector还提供了一些方法，例如根据文件路径获取测试文件、获取测试文件中的测试函数等。

3. DocumentLinkSource: 这个枚举体用来表示代码镜头源文件的类型。代码镜头可以与不同类型的源文件相关联，比如测试文件(`Test`)、文档文件(`Docs`)、模块文件(`Module`)等。DocumentLinkSource定义了不同类型的源文件，以便在代码镜头逻辑中进行区分。

通过这些结构体和枚举体，code_lens.rs文件提供了一种机制可以为源代码文件中的特定位置生成代码镜头。代码镜头通常是一些在代码边缘显示的小组件，可以提供相关命令、信息或者链接。这些代码镜头可以增强开发者在编辑器中的交互体验，提供额外的功能和导航选项。

