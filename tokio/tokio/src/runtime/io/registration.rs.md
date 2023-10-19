# File: tokio/tokio/src/runtime/io/registration.rs

在tokio源代码中，`tokio/tokio/src/runtime/io/registration.rs`文件的作用是处理I/O事件的注册和注销操作。具体来说，该文件包含了`Registration`和`PollRegistration`两个结构体，用于在I/O事件循环中注册和监听特定的文件描述符。

首先，`Registration`结构体表示了一个I/O事件的注册对象。它使用了非阻塞方式注册了一个文件描述符，并添加到一个内部的队列中，以便在事件循环中处理。`Registration`结构体具有以下主要方法和功能：
- `new()`：创建一个新的`Registration`实例。
- `register()`：将文件描述符注册到目标事件循环中。它会调用操作系统特定的方法，如`epoll_ctl`或`kqueue`。
- `reregister()`：重新注册文件描述符，用于更改注册的事件。
- `deregister()`：注销文件描述符，从事件循环中移除。

其次，`PollRegistration`结构体是与`Registration`相关联的另一个对象，它用于在调用`Registration`实例的`register()`方法时创建。`PollRegistration`结构体具有以下主要方法和功能：
- `new()`：创建一个新的`PollRegistration`实例。
- `register()`：将关联的`Registration`添加到目标事件循环的内部队列中。
- `deregister()`：从目标事件循环的内部队列中移除关联的`Registration`。

这些结构体的作用是为了在tokio的事件循环中正确注册和注销I/O事件，以便能够非阻塞地监听和处理特定的文件描述符事件。

