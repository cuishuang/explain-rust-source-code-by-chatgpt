# File: rayon/src/iter/filter_map.rs

在Rust的Rayon库的源代码中，rayon/src/iter/filter_map.rs这个文件的作用是实现了filter_map迭代器适配器。filter_map是一个可以同时对迭代器中的元素进行过滤和映射的操作。

文件中的三个struct`FilterMap<I, FilterMapConsumer<'p, Item>, FilterMapFolder<'p, Item, Out>, Item, Out>`分别有以下作用：

1. `FilterMap<I, FilterMapConsumer<'p, Item>, FilterMapFolder<'p, Item, Out>, Item, Out>`：这是一个迭代器适配器的定义，它接收一个输入迭代器`I`，一个过滤-映射消费者`FilterMapConsumer<'p, Item>`，以及一个过滤-映射折叠器`FilterMapFolder<'p, Item, Out>`。通过实现`std::iter::rayon::ParallelIterator` trait，它可以使得输入迭代器能够并行地进行过滤和映射操作。

2. `FilterMapConsumer<'p, Item>`：这是一个消费者的定义，它负责并行地处理每个元素，并生成输出迭代器所需的结果。它实现了`std::iter::rayon::Consumer` trait，并通过`core::mem::replace`方法来获取每个元素并对其进行过滤和映射的操作，然后将过滤-映射结果存储在一个向量中。

3. `FilterMapFolder<'p, Item, Out>`：这是一个折叠器的定义，它负责对每个元素进行过滤和映射的操作。作为消费者`FilterMapConsumer`的一个参数，它实现了`std::iter::rayon::Folder` trait，并通过实现`Folder::consume`方法来进行过滤和映射的操作。它还实现了`Folder::complete`方法来生成最终的输出迭代器。

这些struct的协作使得filter_map迭代器适配器能够并行地对输入迭代器进行过滤和映射操作，提高了迭代器处理的效率。

