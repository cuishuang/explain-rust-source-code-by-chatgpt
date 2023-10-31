# File: rust-analyzer/crates/rust-analyzer/src/lsp/utils.rs

文件 `rust-analyzer/crates/rust-analyzer/src/lsp/utils.rs` 的作用是提供与 LSP（Language Server Protocol）相关的工具函数和数据结构。它包含了一些函数和结构体，帮助实现 LSP 消息的发送和解析，以及处理 LSP 消息时的一些辅助逻辑。

下面是一些 `utils.rs` 文件中主要内容的介绍：

1. `from_proto::from_proto` 函数：根据 LSP 消息中的数据结构，将其转换为 Rust Analyzer 内部使用的数据结构。例如，将 LSP 中的 `CompletionParams` 结构体转换为 Rust Analyzer 中的 `CompletionParams` 结构体。

2. `to_proto::to_proto` 函数：将 Rust Analyzer 内部的数据结构转换为 LSP 协议中的数据结构。例如，将 Rust Analyzer 中的 `CompletionResult` 结构体转换为 LSP 中的 `CompletionList` 结构体。

3. `notification` 模块：提供了发送不需要返回值的 LSP 通知消息的函数。例如，`semantic_tokens_full` 函数用于发送 `rust-analyzer/semanticTokens/full` 通知。

4. `request` 模块：提供了发送需要返回值的 LSP 请求消息的函数。例如，`text_document/code_action` 函数用于发送 `textDocument/codeAction` 请求。

5. `Progress` 枚举：定义了一些在处理 LSP 请求或通知时的进度状态。其中的枚举项包括：
   - `Started`：表示进度开始。
   - `Report`：表示进度报告。
   - `Done`：表示进度完成。

   这些进度状态可以用于表示复杂操作（例如，代码重构或分析）的进度，使客户端可以了解到长时间运行操作的进展。

总之，`rust-analyzer/crates/rust-analyzer/src/lsp/utils.rs` 文件是一个提供与 LSP 相关的功能的工具模块，它包含了一些用于发送和处理 LSP 消息的函数和数据结构。`Progress` 枚举用于表示进度状态，以提供对长时间运行操作的进度追踪。

