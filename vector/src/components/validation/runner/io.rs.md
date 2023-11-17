# File: vector/src/components/validation/runner/io.rs

在Rust生态的vector项目中，vector/src/components/validation/runner/io.rs文件的作用是定义了与输入和输出相关的结构体和功能。

1. EventForwardService 结构体是一个具有异步发送能力的事件转发服务。它实现了`futures::stream::Stream` trait，可以从输入边获取事件并将其异步转发到指定的输出边。它通常用于处理从输入到输出的事件传递。

2. InputEdge 结构体表示输入边，它定义了从数据源获取事件的方法和一些配置选项。它包含了一个异步任务`input`，该任务会一直监听数据源，不断获取新的事件。并且提供了可自定义的事件处理逻辑，比如过滤、解析等。

3. OutputEdge 结构体表示输出边，它定义了向目标位置发送事件的方法和一些配置选项。它可以将从输入边转发过来的事件发送到指定的目标位置，例如文件、HTTP、Kafka等。它还提供了一些可选的事件处理逻辑，例如转换、格式化等。

4. ControlledEdges 结构体是一个管理输入和输出边的容器。它通过使用`Arc<Mutex<InputEdge>>`和`Arc<Mutex<OutputEdge>>`的方式，来创建可以在多个线程中共享访问的输入和输出边。ControlledEdges可以用于并行地处理输入和输出，同时保证线程安全性。

这些结构体共同组成了向量流水线中数据的输入、输出、事件处理和转发的基础。它们为Vector提供了强大的数据处理和传输能力，并且具有高度的可配置性和可扩展性。

