# File: rust-analyzer/crates/hir-expand/src/lib.rs

rust-analyzer/crates/hir-expand/src/lib.rs这个文件的作用是进行Rust代码的宏展开，将宏调用转换为实际的代码。

HirFileId(u32)定义了一个表示Hir文件的唯一标识符，用于在代码库中跟踪文件。

MacroFile是一个表示宏定义所在文件的结构体，包含了它在Hir文件中的ID和在源文件中的位置信息。

MacroCallId(salsa::InternId)定义了一个表示宏调用的唯一标识符，用于在代码库中跟踪宏调用。

MacroCallLoc表示宏调用在Hir文件中的位置信息。

MacroDefId是一个表示宏定义的唯一标识符，用于在代码库中跟踪宏定义。

EagerCallInfo用于存储宏调用的信息，包括调用位置、宏定义和展开后的代码。

ExpansionInfo用于存储宏展开的信息，记录了宏展开过程中的一些详细信息。

InFile<T>是一个泛型结构体，用于表示T类型在Hir文件中的位置信息。

InMacroFile<T>是一个泛型结构体，用于表示T类型在宏文件中的位置信息。

UnresolvedMacro表示一个未解析的宏。

ExpandError是一个枚举类型，用于表示宏展开过程中可能出现的错误。

MacroDefKind是一个枚举类型，表示宏定义的类型。

MacroCallKind是一个枚举类型，表示宏调用的类型。

HirFileIdRepr是一个枚举类型，表示Hir文件的唯一标识符的不同表示方式。

ExpandTo是一个枚举类型，表示宏展开的目标。

