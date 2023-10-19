# File: tokio/tokio-stream/src/stream_ext/collect.rs

在tokio源代码中，tokio/tokio-stream/src/stream_ext/collect.rs文件的作用是提供对流（Stream）的收集（collect）操作的扩展。

首先，Collect<T, Internal>结构是一个统计信息的容器，用于收集流中的元素并生成一个结果。它具有以下字段和方法：
- items: 一个可变的向量，用于存储流中的元素。
- _priv: 一个Internal类型的字段，用于封装私有的成员。

接下来是FromStream<T>和FromStreamPriv<T>这两个trait。这两个trait定义了从流中收集元素的逻辑。具体来说：
- FromStream<T> trait是提供对Stream的收集操作的公共接口。它有一个collect()方法，接受一个Stream作为参数，并返回一个实现Future trait的Future对象。通过调用collect()方法，可以收集流中的元素并生成一个结果。
- FromStreamPriv<T> trait是FromStream<T> trait的私有接口，具体实现了collect()方法。它使用了Collect<T, Internal>结构来收集流中的元素，并生成一个结果。

总结起来，tokio/tokio-stream/collect.rs文件提供了对流进行收集操作的扩展，其中Collect<T, Internal>结构是用于收集元素的统计容器，而FromStream<T>和FromStreamPriv<T>这两个trait定义了从流中收集元素的逻辑。

