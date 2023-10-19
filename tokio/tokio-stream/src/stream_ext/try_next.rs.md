# File: tokio/tokio-stream/src/stream_ext/try_next.rs

tokio/tokio-stream/src/stream_ext/try_next.rs文件的作用是为实现Stream trait的类型添加一个try_next()方法。这个方法可以尝试从流中获取下一个元素，但不阻塞。

在这个文件中，TryNext<'a, S>结构体是一个Stream扩展器，它接收一个实现了Stream trait的类型S作为输入。该结构体是一个内部封装，用于为S类型添加try_next()方法。它具有以下作用：

1. 实现了Future trait，因此可以用于异步任务。
2. 维护了一个内部状态，用于跟踪从流中获取元素的进度。
3. 提供了try_next()方法，该方法尝试从输入流中获取下一个元素，返回一个包装了Option类型的Future，表示可能的下一个元素。

主要的工作逻辑如下：

1. TryNext结构体内部维护了一个Option<State>类型的值，其中State是一个代表获取元素的进度和结果的枚举类型。初始状态为None。
2. 在try_next()方法中，首先检查状态，如果状态不是None，则直接返回保存的状态。
3. 如果状态是None，则尝试从输入流中获取下一个元素，返回一个Future，表示该操作的结果。
4. 获取元素时，TryNext结构体将自身的引用封装为TryNextFuture，作为一个Future执行。TryNextFuture使用poll()方法来获取元素。
5. 在poll()方法中，如果已经有了保存的状态，将其返回。否则，使用Pin::new_unchecked()将输入流的引用转换为Pin类型，并尝试调用poll_next()方法来获取下一个元素。
6. poll_next()方法返回的是一个包装了Option类型的Future，表示可能的下一个元素。如果这个Future已经可以立即返回结果，则将其解包并返回；否则，将状态保存到TryNext结构体中，并返回。
7. 返回的Future表示获取下一个元素的异步操作。

总结来说，tokio-stream/src/stream_ext/try_next.rs文件中定义了TryNext结构体，用于为实现了Stream trait的类型添加try_next()方法，该方法可以非阻塞地尝试从流中获取下一个元素。TryNext结构体使用TryNextFuture作为一个Future来执行异步任务。

