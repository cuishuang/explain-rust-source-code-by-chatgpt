# File: vector/src/source_sender/mod.rs

在Rust生态中，vector项目是一个可扩展、可靠的数据收集、传输和处理工具。在vector源代码中，vector/src/source_sender/mod.rs文件起到了协调和处理事件发送的作用。

在该文件中，有几个struct起到不同的作用：

1. Builder：Builder结构体是用来构建SourceSender的辅助结构体。它提供了一些方法来设置SourceSender的参数，例如事件缓冲区大小和超时时间等。

2. SourceSender：SourceSender结构体是用来发送事件至pipeline的结构体。它拥有一个channel，用于将事件发送给下游的processors。它还负责处理事件发送超时和处理失败的情况。

3. UnsentEventCount：UnsentEventCount结构体用于跟踪未发送的事件数量。当事件发送失败时，将通过UnsentEventCount进行计数，并在一定时间后重新尝试发送。

4. Inner：Inner结构体是SourceSender的内部实现，它是一个可变的状态结构体。Inner结构体中维护了与事件发送相关的状态，例如缓冲区、事件发送者和事件执行者等。

这些结构体共同协作，实现了事件的缓冲和发送功能，确保向pipeline传递事件的可靠性和高效性。

