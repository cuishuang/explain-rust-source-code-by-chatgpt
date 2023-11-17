# File: rust-clippy/clippy_lints/src/methods/unnecessary_sort_by.rs

在rust-clippy的源代码中，unnecessary_sort_by.rs文件的作用是实现 Clippy的一个 lint规则，用于检测不必要的排序操作。该 lint检查代码中是否存在使用`sort_by(|a, b| a.cmp(b))`的情况，如果存在，则会报出 lint警告。

SortDetection结构体是用来检测代码中的排序操作的，它实现了 rustc的`hir::intravisit::Visitor` trait。在visitor的visit_expr方法中，SortDetection会匹配所有的排序调用，并记录下来。

SortByKeyDetection结构体是用来检测代码中的排序调用是否可以使用`sort_by_key`来代替的。它同样实现了 rustc的`hir::intravisit::Visitor` trait。在visitor的visit_expr方法中，SortByKeyDetection会匹配所有的排序调用，并判断其参数是否可以被视为key，如果可以，则记录下来。

LintTrigger是一个枚举类型，表示 Clippy的 lint触发方式。它有以下几个变体：
- `Method`：触发 lint的方式是使用`sort_by`方法，代码中包含`x.sort_by(|a, b| a.cmp(b))`的形式。
- `Traits`：触发 lint的方式是通过实现`PartialOrd`和`Ord` trait来调用排序方法。
- `IntoIterator`：触发 lint的方式是通过调用`into_iter`方法获取迭代器然后进行排序。
- `Wildcard`：通配符，表示其他未匹配到的情况。

这些枚举变体用于在 lint的报告中标识触发 lint的具体原因，以便开发者知晓代码中不必要的排序操作发生的地方。

