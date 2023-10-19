# File: tokio/tokio/src/fs/dir_builder.rs

在tokio的源代码中，`tokio/src/fs/dir_builder.rs`文件定义了`DirBuilder`结构体及其相关实现。

`DirBuilder`结构体是使用`Tokio`异步文件系统操作创建目录的构建器。它提供了一系列方法和选项，用于自定义目录创建行为。

`DirBuilder`结构体有以下主要方法：

1. `new()`：创建一个新的`DirBuilder`实例。
2. `recursive()`：启用递归模式，当创建目录时，会自动创建父目录（如果父目录不存在）。
3. `mode()`：设置目录的权限模式。
4. `create()`：创建一个新的目录，返回一个`Future`，表示异步操作。
5. `mkdir()`：创建一个新的目录并设置权限模式，返回一个`Future`，表示异步操作。

`recursive()`方法允许调用者创建嵌套目录。如果启用了递归模式并且要创建的目录的父目录不存在，`DirBuilder`会递归创建所有父目录。

`mode()`方法用于设置创建的目录的权限模式。在UNIX系统中，它会设置目录的权限位，而在Windows系统中，该选项会被忽略。

`create()`方法用于创建一个新的目录。它返回一个`Future`，表示异步操作的结果，可以通过await等待结果。

`mkdir()`方法与`create()`方法类似，但同时也设置了目录的权限模式。

总之，`DirBuilder`结构体及其相关方法封装了目录创建的逻辑，使用户可以使用自定义选项进行异步文件系统目录的创建。

