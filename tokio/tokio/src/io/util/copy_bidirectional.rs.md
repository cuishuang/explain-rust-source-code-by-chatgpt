# File: tokio/tokio/src/io/util/copy_bidirectional.rs

在tokio源代码中，tokio/tokio/src/io/util/copy_bidirectional.rs这个文件的作用是实现了一个双向拷贝的工具函数。它允许将数据从一个源（Read）流复制到一个目标（Write）流，并且同时将数据从目标流复制回源流。

该文件中的主要结构是CopyBidirectional，它是一个实现了Future trait的结构。CopyBidirectional包含了源（Read）流和目标（Write）流的句柄，以及一些内部状态和配置。

在CopyBidirectional实现中，流的拷贝是通过一个无限循环来实现的。每个循环迭代，它会首先试图从源流读取数据，并将读取的数据写入目标流。然后再试图从目标流读取数据，并将读取的数据写回源流。这样就实现了双向的数据流动。如果读取或写入操作遇到阻塞，CopyBidirectional就会暂停并等待。一旦读取和写入操作都完成，CopyBidirectional就可以顺利的继续下一个循环迭代。

TransferState是一个枚举类型，它定义了拷贝状态的几个可能取值。这些状态是：

- AwaitingTransfer：表示拷贝操作正在等待进行，还没有开始。
- Reading：表示正在从源流读取数据。
- Writing：表示正在将数据写入目标流。
- Done：表示拷贝操作已经完成。

通过使用TransferState，CopyBidirectional能够跟踪和管理拷贝的状态，以便在每个循环迭代中正确地处理读取和写入操作，并正确地转换状态。

总之，tokio/tokio/src/io/util/copy_bidirectional.rs这个文件实现了一个双向拷贝的工具函数，通过无限循环将数据从源流复制到目标流，并且同时将数据从目标流复制回源流。TransferState枚举类型用于追踪和管理拷贝的状态。

