# File: vector/lib/vector-common/src/internal_event/service.rs

文件vector/lib/vector-common/src/internal_event/service.rs的作用是定义了发送内部事件的服务。内部事件是Vector管道内部使用的事件，它们用于在管道中的各个组件之间进行通信和同步。

该文件中定义了一个名为InternalEventService的结构体，该结构体实现了Sink和Stream trait，表示它可以接收和发送内部事件。它还实现了Future trait，可以用于异步等待内部事件的处理结果。

接下来，让我们详细介绍一下PollReadyError<E>和CallError<E>这两个结构体的作用：

1. PollReadyError<E>：这是一个通用的错误类型，用于表示向内部事件服务发送事件时出现的错误。它包含一个泛型参数E，表示具体的错误信息。该结构体在实现Sink trait时作为返回类型使用，用于表示下一次可以安全地发送事件的时间戳或发送事件时出现的错误。

2. CallError<E>：这也是一个通用的错误类型，用于表示内部事件服务的Future调用过程中出现的错误。它包含一个泛型参数E，表示具体的错误信息。该结构体在内部事件服务的Future实现中作为返回类型使用，用于表示调用结果的成功或失败。

这两个结构体的目的是提供一种机制来处理发送内部事件时可能出现的错误。通过使用这些错误类型，Vector可以在事件发送过程中进行错误处理，并相应地进行流程控制和错误恢复。

