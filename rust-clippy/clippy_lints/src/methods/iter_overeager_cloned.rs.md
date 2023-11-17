# File: rust-clippy/clippy_lints/src/methods/iter_overeager_cloned.rs

文件`iter_overeager_cloned.rs`的作用是实现clippy lint规则，用于检查在遍历迭代器时过度使用`cloned()`方法造成的性能损耗。

在该文件中，定义了`clippy_lints::iter_overeager_cloned`模块，并包含以下结构体和枚举：

1. `MoveDelegate` 结构体：用于处理移动操作时的迭代器委托。
   - 方法 `new()`：创建一个新的 `MoveDelegate` 实例。
   - 方法 `take()`：从 `self` 中获取下一个元素并返回，同时更新内部状态。
   - 方法 `take_and_map()`：从 `self` 中获取下一个元素，执行给定的映射操作，并返回映射结果，同时更新内部状态。

2. `Op<'a>` 枚举：用于表示迭代器操作的不同类型，并在遍历过程中使用。
   - `Leave`：只是移出当前迭代器元素并终止迭代。
   - `Unmodified`：不执行任何操作，继续迭代。
   - `Cloned`：在当前迭代器元素上执行`cloned()`方法并继续迭代。
   - `Borrowed`：在当前迭代器元素上执行`borrowed()`方法并继续迭代。
   - `Consumed`：在当前迭代器元素上执行消费者方法（例如`next()`）并终止迭代。
   - `Moved`：在当前迭代器元素上执行移动操作并终止迭代。

这些结构体和枚举主要用于遍历迭代器并执行相应的操作，通过使用适当的迭代器操作来避免不必要的性能损耗。

