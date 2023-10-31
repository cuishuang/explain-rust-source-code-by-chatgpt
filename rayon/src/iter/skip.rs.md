# File: rayon/src/iter/skip.rs

在Rust的rayon库中，rayon/src/iter/skip.rs文件的作用是实现`Skip`迭代器适配器。`Skip`适配器会跳过原始迭代器的前`n`个元素，然后返回剩余的元素。

`struct Skip<I>`代表`Skip`适配器。它实现了`rayon::iter::plumbing::Producer`特质，该特质定义了Rayon框架中的迭代器模型。`Skip`包含原始迭代器`iter`和要跳过的元素个数`skip`, 它的主要作用是控制和跟踪迭代的状态，并在需要时产生元素。

`Skip`的核心方法是`drive_unindexed<Cb>(self, callback: CB)`，它接收一个`Cb`类型的callback，并根据跳过的元素个数和原始迭代器的具体实现来推动迭代的进行。`drive_unindexed`方法使用了迭代器模型中的`unindexed`方法来实现。

`struct Callback<CB>`是一个闭包trait，提供了对产生`Skip`适配器中元素的处理。具体实现则是由用户自己定义的闭包来完成。通过闭包，用户可以对`Skip`适配器中的每个元素进行操作或处理。

总之，`Skip`适配器允许用户在迭代过程中跳过头部元素，并返回剩余的元素。`Skip`的核心方法是`drive_unindexed`，它负责实现迭代的推进和驱动，并通过闭包`Callback`对每个元素进行处理。

