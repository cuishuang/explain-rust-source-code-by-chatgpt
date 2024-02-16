# File: /Users/fliter/rust-contribute/deno/cli/lsp/tsc.rs

在Deno项目的源代码中，/Users/fliter/rust-contribute/deno/cli/lsp/tsc.rs文件的作用是实现TypeScript的LSP（Language Server Protocol）接口，提供与TypeScript编译器通信所需的请求和响应。

以下是该文件中的一些struct的作用说明：
- FormatCodeSettings: 包含了代码格式化的设置选项，例如缩进、分号等。
- TsServer: TypeScript服务器，是TypeScript编译器的代理，用于处理来自客户端的请求，并调用TypeScript编译器进行代码分析和处理。
- DroppableToken(CancellationToken): 可以被取消的令牌，用于取消当前的操作。
- AssetDocumentInner: 资源文档的内部表示，包含了文档的内容、版本等相关信息。
- AssetDocument(Arc<AssetDocumentInner>): 资源文档，包含了一个Arc指向内部表示，并提供了对文档内容的读取和更新操作。
- AssetsSnapshot(Arc<Mutex<AssetsMap>>): 资源快照，包含了一个Arc指向AssetsMap的互斥锁，用于提供对所有资源文档的读取操作。
- Assets: 资源管理器，用于管理所有的资源文档。
- TextSpan: 文本范围，用于表示在文档中的一个范围。
- SymbolDisplayPart: 符号显示部分，用于表示在代码中的一个符号及其相关信息。
- JsDocTagInfo: JSDoc标签信息，用于表示JSDoc中的一个标签。
- QuickInfo: 快速信息，用于表示代码的快速信息提示。
- Link: 链接，用于表示代码的一个跳转链接。
- DocumentSpan: 文档范围，表示代码中的一个范围，并提供了对该范围的进一步操作。
- NavigateToItem: 导航项，表示在代码中的一个导航目标。
- InlayHint: 内部提示，用于在代码中显示内部的信息提示。
- NavigationTree: 导航树，用于表示代码的导航树结构。
- ImplementationLocation: 实现位置，表示一个代码实现的位置。
- RenameLocation: 重命名位置，表示一个代码重命名的位置。
- RenameLocations: 重命名位置集合，表示一个代码重命名的位置集合。
- HighlightSpan: 高亮范围，表示代码的一个高亮范围。
- DefinitionInfo: 定义信息，用于表示代码的定义信息。
- DefinitionInfoAndBoundSpan: 定义信息和绑定范围，表示代码的定义信息以及其绑定的范围。
- DocumentHighlights: 文档高亮，用于表示代码中的高亮信息。
- TextChange: 文本修改，表示对文本进行的修改操作。
- FileTextChanges: 文件文本修改，表示对文件文本进行的修改操作。
- Classifications: 代码分类，用于表示代码的不同分类。
- RefactorActionInfo: 重构动作信息，用于表示重构动作的信息。
- ApplicableRefactorInfo: 可应用的重构信息，用于表示可以应用的重构信息。
- RefactorEditInfo: 重构编辑信息，用于表示重构编辑的信息。
- CodeAction: 代码动作，用于表示对代码进行的动作。
- CodeFixAction: 代码修复动作，用于表示对代码进行的修复动作。
- CombinedCodeActions: 组合代码动作，用于表示多个代码动作的组合。
- ReferencedSymbol: 引用的符号，在代码中表示引用了某个符号。
- ReferencedSymbolDefinitionInfo: 引用符号的定义信息，表示引用符号的定义信息。
- ReferencedSymbolEntry: 引用符号条目，表示引用符号的条目信息。
- ReferenceEntry: 引用条目，表示代码中的一个引用条目。
- CallHierarchyItem: 调用层次项，表示在代码中的一个调用级别项。
- CallHierarchyIncomingCall: 调用层次中的传入调用，表示一个传入的调用。
- CallHierarchyOutgoingCall: 调用层次中的传出调用，表示一个传出的调用。
- CompletionEntryDetails: 自动完成项的详细信息，表示自动完成项的详细信息。
- CompletionInfo: 自动完成信息，表示代码的自动完成信息。
- CompletionItemData: 自动完成项的数据，表示自动完成项的相关数据。
- CompletionEntryDataImport: 自动完成项的引入数据，表示自动完成项的引入数据。
- CompletionEntry: 自动完成项，表示代码的一个自动完成项。
- CompletionEntryLabelDetails: 自动完成项的标签详细信息，表示自动完成项的标签的详细信息。
- OutliningSpan: 折叠范围，表示代码的一个折叠范围。
- SignatureHelpItems: 签名帮助项，用于表示代码的签名帮助信息。
- SignatureHelpItem: 签名帮助条目，表示代码的签名帮助条目。
- SignatureHelpParameter: 签名帮助参数，表示代码的签名帮助参数。
- SelectionRange: 选择范围，表示代码的一个选择范围。
- Response: LSP响应，表示与客户端交互的响应数据。
- TscSpecifierMap: TypeScript服务器的规范映射，用于管理TypeScript服务器的规范。
- State: LSP服务的状态，用于管理服务器的状态。
- LoadResponse: 加载响应，表示加载请求的响应数据。
- GetCompletionsAtPositionOptions: 获取指定位置的自动完成选项，表示获取自动完成选项的请求参数。
- UserPreferences: 用户首选项，用于表示用户的首选项配置。
- SignatureHelpItemsOptions: 签名帮助项的请求参数，用于指定获取签名帮助项的选项。
- SignatureHelpTriggerReason: 签名帮助触发原因，表示导致获取签名帮助的原因。
- GetCompletionDetailsArgs: 获取自动完成项的详细信息的请求参数，用于指定获取自动完成项详细信息的选项。
- GetNavigateToItemsArgs: 获取导航项目的请求参数，用于指定获取导航项目的选项。
- TscRequest: TypeScript编译器的请求，用于发送给TypeScript编译器的请求。

以下是该文件中的一些enum的作用说明：
- IndentStyle: 缩进风格，用于表示代码的缩进样式。
- SemicolonPreference: 分号偏好设置，用于表示代码的分号偏好。
- OneOrMany<T>: 用于表示可能为单个值或多个值的情况。
- ScriptElementKind: 脚本元素种类，用于表示代码中的元素种类。
- MatchKind: 匹配种类，用于表示匹配的种类。
- InlayHintKind: 内部提示种类，用于表示内部提示的种类。
- HighlightSpanKind: 高亮范围种类，用于表示高亮范围的种类。
- OutliningSpanKind: 折叠范围种类，用于表示折叠范围的种类。
- CompletionTriggerKind: 自动完成触发种类，用于表示自动完成的触发种类。
- ImportModuleSpecifierEnding: 导入模块指定器的结尾，用于表示导入模块指定器的结尾方式。
- IncludeInlayParameterNameHints: 是否包含内部参数名提示，用于表示是否包含内部参数名提示。
- IncludePackageJsonAutoImports: 是否包含package.json的自动导入，用于表示是否包含package.json的自动导入。
- SignatureHelpTriggerKind: 签名帮助触发种类，用于表示签名帮助的触发种类。

这些struct和enum的定义提供了对TypeScript编译器相关功能的封装和管理，根据不同的请求类型和参数，可以通过这些定义来进行代码的分析、操作和处理。

