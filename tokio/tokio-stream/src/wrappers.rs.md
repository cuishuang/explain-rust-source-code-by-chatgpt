# File: tokio/tokio-stream/src/wrappers.rs

tokio-stream库提供了一组实用程序来处理异步流。其中tokio/tokio-stream/src/wrappers.rs文件包含了一些针对流类型的包装器。

该文件中定义了几个重要的结构和函数。以下是这些结构和函数的详细介绍：

1. `InPlace<T>`：这是一个可持有类型`T`的包装器。它实现了Stream trait，并在Stream上实施了一些特定行为。当从Stream中获取下一个元素时，它会将元素值直接移出Stream，而不是创建新的内存分配。这对于某些用例可以提供性能优势。

2. `AndStream<A, B>`：这是A和B两个流的逻辑"与"操作。它实现了Stream trait，每次从A和B中获取下一个元素，然后将其返回。如果A或B的流结束，它将停止在另一个流上获取元素。

3. `OrStream<A, B>`：这是A和B两个流的逻辑"或"操作。它实现了Stream trait，并在A和B中任何一个上获取元素。如果A和B的流都结束，它将停止获取元素。

4. `Once<T>`：这是一个只返回一次元素的流。它实现了Stream trait，并在第一次请求元素时返回已存储的元素。在获取一次元素后，它将返回`None`，表示流已结束。

5. `ReuniteErr<A, B>`：这个结构将A和B两个流重组成一个流，并使用`Result`类型对错误进行标记。它实现了Stream trait，并将A和B流上的元素返回。当A和B的流返回错误时，它将将该错误转换为`Result`类型并返回。

在tokio中，这些包装器提供了对流进行转换和合并的功能。它们可以帮助开发者更方便地组合和操作异步流。这些包装器是为了增加Stream的灵活性和可用性，并且可以根据不同需求进行组合使用。

