# File: /Users/fliter/rust-contribute/deno/cli/lsp/urls.rs

在Deno项目的源代码中，`/Users/fliter/rust-contribute/deno/cli/lsp/urls.rs`文件的作用是定义用于处理LSP（Language Server Protocol）URL的相关功能。

首先，`LspClientUrl(Url)`结构体是一个包装了URL的结构体，用于表示与客户端通信的URL。

`LspUrlMapInner`结构体是一个内部辅助结构体，它维护了一个映射，将LSP中的URI（Uniform Resource Identifier）映射到内部URL。它提供了方法来获取和修改内部URL的映射关系。

`LspUrlMap`结构体是对`LspUrlMapInner`的封装，它是整个URL映射的管理者。它提供了一系列方法来添加、删除和查询URL的映射关系。

`LspUrlKind`枚举是URL的种类的表示，它定义了不同种类的URL，包括`System`（系统URL），`Client`（客户端URL）和`Vfs`（虚拟文件系统URL）。这个枚举用于指示URL的用途和属性。

总结起来，`urls.rs`文件中的这些结构体和枚举提供了用于管理和处理LSP URL的相关功能，包括URL映射和区分不同种类的URL。

