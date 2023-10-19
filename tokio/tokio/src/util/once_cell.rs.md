# File: tokio/tokio/src/util/once_cell.rs

tokio/tokio/src/util/once_cell.rs 是 Tokio 中的一个工具模块，提供了一种以最小的开销在多个异步任务之间共享可变状态的机制。

OnceCell<T> 是一个包含可变值的类型，但是它的主要特点是只能初始化一次。OnceCell<T> 的主要作用是在多个异步任务中共享并延迟初始化一个值，确保只有一个任务成功初始化该值。OnceCell<T> 具有以下几个主要的方法：

1. `pub fn set(&self, value: T) -> Result<(), T>`：尝试设置 OnceCell 的值。如果值已被设置，则返回包含原始值的错误。如果成功设置值，返回 `Ok(())`。

2. `pub fn get(&self) -> Option<&T>`：获取 OnceCell 的值的不可变引用。如果值尚未初始化，则返回 `None`。

3. `pub fn get_mut(&mut self) -> Option<&mut T>`：获取 OnceCell 的值的可变引用。如果值尚未初始化或已被借用，返回 `None`。

4. `pub fn into_inner(self) -> Option<T>`：将 OnceCell 的值取出并返回，不更新 OnceCell。如果值尚未初始化，则返回 `None`。

OnceCell<T> 的设计目标是避免资源竞争和数据竞争，保证一致性和正确性。它基于内部的原子操作和标志位，以确保只有一个线程（或任务）能成功初始化并访问该值。而其他线程或任务只能获取共享的不可变引用，或者等待初始化完成后再访问。

通过使用 OnceCell<T>，Tokio 的用户可以利用异步任务之间共享可变状态的机制，而不需要手动进行同步和锁操作，从而简化了并发编程的复杂性。

