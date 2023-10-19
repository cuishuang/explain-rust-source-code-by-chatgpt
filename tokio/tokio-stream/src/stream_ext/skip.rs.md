# File: tokio/tokio-stream/src/stream_ext/skip.rs

tokio-stream是Tokio库中的一个模块，提供了异步流的实现。

而skip.rs是stream_ext模块下的文件，它定义了一个名为Skip的结构体和其他相关的实现。其主要作用是为流(Stream)添加了跳过（skip）操作。

首先，Skip<St>结构体是一个泛型结构体，它包装了一个流（Stream）对象St。Skip结构体实现了Stream trait，因此可以像操作普通的流一样操作Skip。

Skip结构体拥有一些方法，其中最重要的是skip和skip_while方法。

1. skip方法接收一个usize类型的参数n，并返回一个新的Skip对象。该对象将会跳过前n个元素，然后产生剩余的元素。

2. skip_while方法接收一个谓词函数predicate，并返回一个新的Skip对象。该对象会跳过满足谓词函数的前缀元素，然后产生剩余的元素。谓词函数会接收流中的元素，并返回一个bool值。

这些方法使得对流的跳过操作变得方便。Skip结构体可以与其他流操作一起使用，例如map、filter等，从而构建更复杂的流处理逻辑。

总结而言，tokio-stream/src/stream_ext/skip.rs文件中的Skip结构体和相关实现，为Tokio库中的异步流提供了跳过操作的功能。

