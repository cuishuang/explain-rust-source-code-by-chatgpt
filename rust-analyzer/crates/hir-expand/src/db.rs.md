# File: rust-analyzer/crates/hir-expand/src/db.rs

在rust-analyzer的源代码中，位于 `rust-analyzer/crates/hir-expand/src/db.rs` 文件中，主要实现了用于处理宏展开的相关功能。下面详细介绍一下各个结构体、特质和枚举的作用。

**DeclarativeMacroExpander** 是一个结构体，用于处理声明式宏的展开。它实现了 `MacroExpander` 特质，提供了一种声明式的方式来指定宏的展开规则，通过解析宏的输入和规则，可以将宏展开为对应的代码。

**ExpandDatabase** 是一个特质，定义了数据库的扩展方法。其中包括了处理宏展开的相关方法。这些方法可以将宏展开成对应的代码，以便后续的处理和分析。

**MacroExpander** 是一个特质，定义了宏展开器的接口。它包含了一系列在展开宏过程中需要使用到的方法，包括解析宏规则、展开宏等。

**TokenExpander** 是一个枚举类型，用于表示不同种类的宏展开器。它定义了宏展开的常见操作，如将宏展开为标记流、将标记流转换为语法树结构等。

枚举类型 `TokenExpander` 中的各个值分别表示了不同的宏展开行为。例如，`MacroRules(TokenExpanderMacroRules)` 表示将宏展开为宏规则，`Ast(TokenExpanderAst)` 表示将宏展开为抽象语法树。

通过使用这些结构体、特质和枚举，可以实现对于不同类型的宏展开操作的处理和扩展，使得用户可以根据自身的需求来定制和实现自己的宏展开逻辑。

