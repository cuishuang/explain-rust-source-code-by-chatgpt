# File: rayon/rayon-demo/src/map_collect.rs

rayon-demo/src/map_collect.rs文件是rayon示例中的一个示例程序，用于展示rayon库中的map_collect方法的用法和性能。

在该文件中，首先定义了一个简单的结构体MapCollectDemo，该结构体包含两个字段：`counter`和`data`。然后，实现了MapCollectDemo的FromIterator trait，用于从迭代器中构建MapCollectDemo对象。

接着，定义了一个main函数作为程序入口。在main函数中，首先通过命令行参数获取线程数和元素数量，然后生成一个产生斐波那契数列的迭代器。之后，使用rayon库的`iter()`方法将迭代器转换为并行迭代器，并调用`map_collect()`方法对迭代器中的每个元素进行处理，最后将处理结果收集到一个新的vector中。

在处理元素的过程中，示例程序计算了每个斐波那契数列元素对应的质数。质数判断的过程通过一个辅助函数实现。如果元素是质数则返回Some(value)，否则返回None。在质数的判断过程中，使用了rayon库的`par_iter()`方法进行并行计算。

最后，示例程序输出了处理结果的长度、质数个数和计算耗时。

该示例程序通过对斐波那契数列进行质数判断的展示，演示了rayon库的map_collect方法在并行计算中的高效性能和易用性。同时，也展示了rayon库的一些其他特性，如将迭代器转换为并行迭代器，以及使用`par_iter()`方法进行并行计算等。

