# File: tokio/tokio-stream/src/once.rs

在Tokio源代码中，`tokio/tokio-stream/src/once.rs`这个文件中包含了一个名为`Once`的结构体和相关的类型与实现。

`Once`结构体是一个异步的、惰性的计算单元。它封装了一次性计算的逻辑，确保计算只会被执行一次，而后使用缓存的结果。`Once`的相关类型和结构体主要有以下几个：

- `Once`：主要结构体，用于封装一次性计算的逻辑。
- `OnceState`：用来描述计算的状态。有两个可能的取值，`INIT`表示计算尚未开始，`COMPLETED`表示计算已经完成。
- `OnceError`：用于表示在计算过程中可能出现的错误。
- `OnceFuture`：一个实现了`Future` trait的结构体，表示封装了一次性计算的未来值。

`Once`结构体的作用是确保一段代码只会被执行一次，主要用于懒加载和初始化操作。在多线程/并发的场景下，`Once`可以保证线程安全，确保计算只会被执行一次，避免重复计算和竞态条件。

`Once`的核心逻辑是通过`std::sync::Once`结构体进行实现的。这里使用`std::sync::Once`为基础，构建了一个异步版本的`Once`结构。

`Once`结构体包含了一个内部状态`state`，它使用`Arc`进行共享。在计算开始之前，状态为`INIT`，一旦计算完成，状态会变为`COMPLETED`。当调用`Once::call_once`方法时，会尝试获取`state`的锁，检查状态是否为`INIT`。如果是，就执行用户提供的计算函数，并将状态变为`COMPLETED`；如果状态已经为`COMPLETED`，则直接返回缓存的结果。

`OnceFuture`是`Once`结构体返回的未来值，它实现了`Future` trait，因此可以使用`await`进行等待。在`OnceFuture`的实现中，会检查`Once`的状态，如果状态为`COMPLETED`，则返回计算的结果；如果状态为`INIT`，则通过`notify`方法将当前的任务注册为等待者，并返回`Poll::Pending`，等待计算结果的完成。

综上所述，`tokio/tokio-stream/src/once.rs`中的`Once`结构体及相关类型和实现主要用于实现一次性计算的逻辑，确保计算只会被执行一次，并提供了异步版本的特性。

