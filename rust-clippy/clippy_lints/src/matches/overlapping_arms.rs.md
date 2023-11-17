# File: rust-clippy/clippy_lints/src/matches/overlapping_arms.rs

文件`overlapping_arms.rs`的作用是检查match语句中的重叠分支。在Rust中，match语句用于匹配一个值的不同模式并执行相应的代码块。然而，如果存在重叠的模式，可能会导致意外行为或错误。

该文件的函数`check`是该lint的入口点，它会检查重叠的模式分支，并发出警告或错误。

现在让我们详细介绍文件中的一些结构和枚举：

1. `SpannedRange<T>`：这是一个存储范围的结构体，其中包括开始和结束的位置(`start`和`end`)，以及相应的值(`start_value`和`end_value`)。它用于表示匹配语句中每个分支的范围。

2. `RangeBound<'a>`：这是一个枚举，表示范围的边界。它可以是一个具体的值(`Included`)，一个开区间(`Excluded`)，或者是没有值(`Unbounded`)。它用于表示范围的开始和结束边界。

3. `EndBound<T>`：这是一个枚举，表示范围的结束边界类型。它可以是一个具体的值(`Included`)，或者是一个开区间(`Excluded`)。

4. `BoundKind`：这是一个枚举，表示范围的边界种类。它可以是一个具体的值(`Included`)，一个开区间(`Excluded`)，或者是没有值(`Unbounded`)。

这些结构和枚举用于表示和处理匹配语句中的范围，并且帮助进行重叠分支的检查和处理。

