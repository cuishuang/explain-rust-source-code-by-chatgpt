# File: rust-analyzer/crates/mbe/src/tt_iter.rs

在rust-analyzer的源代码中，rust-analyzer/crates/mbe/src/tt_iter.rs文件实现了`TtIter`结构体，它在宏展开中用于迭代和处理Token Tree。

`TtIter`结构体是一个惰性（lazy）的迭代器，用于将Token Tree转换为一个可迭代的Token流。它接收一个Token Tree并根据它的结构，将每个Token生成一个`TtToken`对象，然后依次返回给调用者。

`TtIter`结构体的内部包含以下几个重要的字段和方法：
1. `parsed`字段：保存了已解析的Token Tree。
2. `current`字段：指向当前正在处理的Token。
3. `skipped_plus`字段：用于跟踪连续的`Op::SubSep`操作符的数量，以便正确处理结构体字段的语法。
4. `op_queue`字段：保存了下一个Token的`Op`操作符，用于支持语法结构中的分隔符。
5. `lookahead`字段：保存了下一个Token，用于预读Token以确定下一个Token的类型，例如分隔符。
6. `lookahead_valid`字段：指示`lookahead`字段是否包含有效的Token。
7. `next()`方法：用于获取下一个Token，返回一个Option类型。

`TtToken`结构体表示一个Token，包含以下字段：
1. `kind`字段：保存Token的类型，例如标识符、关键字或操作符。
2. `text`字段：保存Token的文本表示。
3. `single`字段：指示Token是否是一个孤立的标记。
4. `spacing`字段：保存Token的前后的空格。

通过这些字段和方法，`TtIter`可以按顺序将Token Tree中的每个Token转换为一个`TtToken`对象，并提供给调用者使用。

总而言之，`TtIter`结构体及其相关的代码实现了将Token Tree转换为可迭代Token流的功能，并提供了一些辅助方法和状态跟踪，以支持宏展开的逻辑。

