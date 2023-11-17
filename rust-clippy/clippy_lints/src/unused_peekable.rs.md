# File: rust-clippy/clippy_lints/src/unused_peekable.rs

在rust-clippy的源代码中，当启用了unused_peekable lint时，unused_peekable.rs文件实现了对`Peekable`类型的检查。`Peekable`是Rust标准库中的一个迭代器适配器，允许在迭代过程中获取下一个元素而不将其消费掉。

unused_peekable.rs文件中的`PeekableVisitor`结构体是一个lint访问者，它实现了`LateLintPass` trait，用于检查`Peekable`类型的使用情况。`PeekableVisitor`结构体是一个具体的访问者对象，它会被lint框架调用以获取待检查的代码和相关的信息。

`PeekableVisitor`结构体中的字段包括：

1. `context`: 一个`LateContext`结构体的引用，提供了对代码上下文的访问，包括AST、Hir、Ty等。
2. `nodetype_to_check`: 一个运行时确定的节点类型，表示要检查的节点类型。
3. `non_macro_paths`: 用于存储路径（如模块路径、函数路径等）的向量。

`PeekableVisitor`结构体有以下几个重要的方法：

1. `new`: 创建一个新的`PeekableVisitor`对象，初始化了`non_macro_paths`等字段。
2. `check_expr`: 在访问表达式时被调用，用于检查表达式中是否使用了`Peekable`类型。
3. `check_path`: 在访问路径时被调用，用于将路径添加到`non_macro_paths`中。

`PeekableVisitor`结构体的实例会被传递给Rust编译器的lint框架，然后在编译过程中拦截和检查匹配lint规则的代码。

总之，unused_peekable.rs文件中的`PeekableVisitor`结构体实现了对`Peekable`类型使用情况的lint检查，通过遍历语法树和相关信息，判断是否存在未使用的`Peekable`类型，如果存在，则给出相应的lint警告或错误。

