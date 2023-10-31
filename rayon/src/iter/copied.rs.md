# File: rayon/src/iter/copied.rs

在Rust的rayon库中，rayon/src/iter/copied.rs文件是用来提供`Copied`迭代器适配器的实现。

`Copied`适配器用于将一个迭代器的值进行复制并返回一个新的迭代器。这在处理需要复制值的情况下非常有用，例如在处理引用类型的迭代器时，可以复制值并将其移动到一个新的线程中。

下面是`Copied`适配器的定义：

```rust
pub struct Copied<I> {
    it: I,
}

impl<I> Copied<I> {
    fn new(it: I) -> Copied<I> {
        Copied { it }
    }
}
```

`Copied`适配器实现了`ParallelIterator` trait，它可以被rayon库的并行迭代器使用。它使用`Callback`、`CopiedProducer`、`CopiedConsumer`和`CopiedFolder`这几个结构体来执行和处理并行计算。

- `Callback<CB>`：这个struct用于表示在迭代器上的每个处理过程中的回调函数。它可以在每个迭代步骤之前或之后执行自定义的代码。

- `CopiedProducer<P>`：这个struct用于生成一个迭代器上的任务，它实现了`Producer` trait。它会从底层迭代器中获取元素，并对每个元素进行复制，最后返回一个可并行处理的任务。

- `CopiedConsumer<C>`：这个struct用于消费由`CopiedProducer`生成的任务的结果。它实现了`Consumer` trait，它将处理结果进行收集和组合。

- `CopiedFolder<F>`：这个struct用于接收来自`CopiedConsumer`的结果，并将它们集成到最终的输出值中。它实现了`Folder` trait。

这些结构体通过`Copied`适配器将迭代器的元素进行复制，然后在并行计算过程中对元素进行处理、收集和组合，最终生成输出结果。


