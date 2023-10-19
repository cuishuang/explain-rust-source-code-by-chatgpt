# File: tokio/tokio-stream/src/stream_ext/filter_map.rs

在tokio源代码中，tokio-stream/src/stream_ext/filter_map.rs文件的作用是实现了`FilterMap`结构体，该结构体提供了对流进行筛选和映射操作的方法。

具体而言，`FilterMap`结构体是用于对流进行筛选和映射操作的组合器（combinator），它使用一个闭包来判断流中的元素是否应该通过筛选，并为通过筛选的元素执行映射操作。`FilterMap`是由类型参数`St`和闭包参数`F`组成。

`FilterMap`结构体实现了`Stream` trait，因此可以将它应用于任何实现了`Stream` trait的类型。它没有自己的状态，因此可以持续对包装的流进行筛选和映射操作。

`FilterMap`结构体提供了以下几个方法：

1. `new(stream: St, f: F) -> FilterMap<St, F>`：创建一个新的`FilterMap`实例，接受一个流和一个闭包作为参数。
2. `poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<F::Output>>>`：用于获取流中的下一个元素，并将其应用于闭包函数。返回值是一个`Poll`枚举类型，表示操作的结果。
3. `size_hint(&self) -> (usize, Option<usize>)`：返回流的大小估计。这个估计是一个元组，表示流的最小和最大大小。
4. `get_pin_mut(&mut self) -> Pin<&mut Self>`：返回一个`Pin`指向`FilterMap`实例的可变引用。
5. `get_mut(&mut self) -> &mut Self`：返回一个可变引用，指向`FilterMap`实例。

总之，`FilterMap`结构体提供了一种流处理的方式，可以在保留原始流不变的情况下对其进行筛选和映射操作。

