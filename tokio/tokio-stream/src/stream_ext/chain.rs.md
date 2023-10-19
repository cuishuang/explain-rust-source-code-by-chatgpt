# File: tokio/tokio-stream/src/stream_ext/chain.rs

在tokio的源代码中，tokio-stream包中的chain.rs文件主要用于实现Stream扩展的chain方法。

该文件定义了名为Chain的结构体，它是一个Stream扩展(`StreamExt`)的实现。Chain结构体有三个类型参数：T，U和I，分别表示原始流的元素类型，下一个流的元素类型和最终链流的元素类型。

Chain结构体实现了Stream trait，提供了一种连接(current)流和下一个流(next)的方法。在调用chain方法时，传入一个当前流和一个next函数，next函数返回一个新的流。然后，Chain结构体利用这两个流生成一个新的链流。

Chain结构体包含以下字段：
1. current_flow: Option<T> - 当前流的元素，用Option类型包装，当为None时表示当前流已经遍历完毕。
2. next_fn: I - 函数类型，用于生成下一个流。
3. next_flow: Option<I> - 下一个流的元素，用Option类型包装，当为None时表示下一个流已经遍历完毕。
4. next: Option<U> - 下一个流。

Chain结构体实现了Stream trait的poll_next方法，用于遍历当前流和下一个流的元素。当当前流还有元素时，会将元素返回给调用者，并将当前流的元素缓存起来。当当前流遍历完毕时，使用next函数生成下一个流并将其赋给next字段，并将下一个流的元素缓存起来。当下一个流也遍历完毕时，返回None，表示遍历结束。

总结一下，tokio-stream中chain.rs文件定义了Chain结构体，它是用于连接不同流的扩展方法。通过调用chain方法，并传入当前流和next函数，可以生成一个新的链流。Chain结构体实现了Stream trait，用于遍历当前流和下一个流的元素，直到遍历结束。

