# File: tokio/tokio/src/util/rc_cell.rs

文件`rc_cell.rs`定义了名为`RcCell`的结构体。`RcCell<T>`是一个具有共享所有权和可变借用的可变单元类型。

在Rust中，原生的`Rc`类型提供了共享所有权，允许多个所有者引用相同的值。然而，`Rc`类型并不允许对所包含的值进行可变借用（borrow mutably）。`RcCell`则通过结合`Rc`和内部可变性（Interior Mutability）来解决这个问题。

`RcCell<T>`是一个包含单个值的容器。它允许多个`Rc`引用共享值，并利用内部可变性来提供可变借用。这意味着，即使拥有共享引用的多个线程都可以读取值，但只有一个线程可以通过`RcCell`进行可变借用，确保在同一时间只有一个线程可以修改值。

`RcCell`提供了以下几个主要的方法：

- `new(value: T) -> Self`：创建一个新的`RcCell<T>`实例，初始值为`value`。
- `borrow(&self) -> RcBorrow<T>`：以只读方式借用`RcCell`中的值，返回一个`RcBorrow<T>`实例，它允许对值进行只读操作。
- `borrow_mut(&self) -> RcBorrowMut<T>`：以可变方式借用`RcCell`中的值，返回一个`RcBorrowMut<T>`实例，它允许对值进行可变操作。
- `try_borrow(&self) -> Option<RcBorrow<T>>`：尝试以只读方式借用`RcCell`中的值，如果值正在可变借用中，则返回`None`。
- `try_borrow_mut(&self) -> Option<RcBorrowMut<T>>`：尝试以可变方式借用`RcCell`中的值，如果值正在借用中（无论是只读还是可变），则返回`None`。

`RcBorrow<T>`是一个提供只读访问的句柄，可以随时访问`RcCell`中的值，但不能进行修改。类似地，`RcBorrowMut<T>`是一个提供可变访问的句柄，允许对值进行修改。

通过使用`RcCell`，我们可以在具有共享引用的同时，通过内部可变性进行值的修改。这在异步编程中非常有用，特别是在多个任务对同一数据进行操作的情况下。

