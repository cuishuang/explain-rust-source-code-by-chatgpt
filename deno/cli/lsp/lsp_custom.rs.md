# File: /Users/fliter/rust-contribute/deno/cli/lsp/lsp_custom.rs

在Deno项目的源代码中，/Users/fliter/rust-contribute/deno/cli/lsp/lsp_custom.rs文件的作用是实现了与Language Server Protocol (LSP) 相关的自定义结构体和枚举，用于LSP服务器与客户端之间的通信。

具体来说，该文件中定义了以下结构体的作用：

- TaskDefinition：表示一个LSP任务的定义，包含了任务的名称和详细描述。
- RegistryStateNotificationParams：表示LSP注册表的状态通知参数，用于向客户端报告LSP服务器支持的功能。
- VirtualTextDocumentParams：表示虚拟文档的参数，用于在LSP服务器中处理虚拟文档的相关操作。
- DiagnosticBatchNotificationParams：表示诊断批处理通知的参数，用于向客户端发送一批诊断信息。
- DenoConfigurationChangeEvent：表示Deno配置更改事件，用于表示Deno的配置发生变化。
- DidChangeDenoConfigurationNotificationParams：表示Deno配置更改通知的参数，用于向客户端发送Deno配置更改的通知。
- UpgradeAvailable：表示Deno有新版本可用的通知，用于向客户端报告Deno有新的升级可用。
- DidUpgradeCheckNotificationParams：表示Deno升级检查通知的参数，用于向客户端发送Deno升级检查的通知。

此外，该文件中还定义了以下枚举的作用：

- RegistryStateNotification：表示LSP注册表的状态通知，用于指示LSP服务器支持的功能。
- DenoConfigurationChangeType：表示Deno配置更改的类型，可以是新增、修改或删除配置。
- DenoConfigurationType：表示Deno配置的类型，可以是“用户”配置或“工作区”配置。
- DidChangeDenoConfigurationNotification：表示Deno配置更改通知，用于向客户端发送Deno配置更改的通知。
- DidUpgradeCheckNotification：表示Deno升级检查通知，用于向客户端发送Deno升级检查的结果。
- DiagnosticBatchNotification：表示诊断批处理通知，用于向客户端发送一批诊断信息。

这些自定义的结构体和枚举在Deno的LSP服务器中扮演了重要的角色，用于描述和传递LSP相关的信息和通知。

