# File: /Users/fliter/rust-contribute/deno/cli/lsp/performance.rs

在Deno项目的源代码中，/Users/fliter/rust-contribute/deno/cli/lsp/performance.rs文件的作用是提供性能度量相关的实用工具和数据结构。

该文件中定义了以下几个结构体：

1. PerformanceAverage: 这个结构体用于计算某个操作的平均执行时间。它包含了一个计时器和计数器，可以通过调用`record`方法来记录每次操作的执行时间，并据此更新平均时间。

2. PerformanceMark: 这个结构体表示一个性能度量点，在代码中使用`Performance.measureStart`和`Performance.measureEnd`方法来记录一个操作的开始和结束，从而计算出其执行时间。

3. PerformanceMeasure: 这个结构体用于记录某个操作的性能度量，包括名称、开始时间、结束时间和执行时间等信息。

4. Performance: 这个结构体提供了一组性能度量的方法，包括开始/结束度量、记录度量结果、计算平均执行时间等。它内部包含了一个HashMap，用于保存每个度量点的具体信息。

这些工具和数据结构可以帮助开发者对代码的性能进行评估和优化。通过使用这些结构体，开发者可以测量某个操作的执行时间，并得到平均执行时间，从而找到代码中耗时较长的部分，以便做出相应的改进和优化。这对于提升系统的性能非常重要。

