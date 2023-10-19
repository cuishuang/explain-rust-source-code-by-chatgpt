# File: tokio/tokio/src/signal/windows/sys.rs

tokio/tokio/src/signal/windows/sys.rs 是 tokio 框架中用于处理 Windows 系统信号的模块。该模块通过与操作系统的系统调用交互，允许 Tokio 应用程序在 Windows 上处理信号。

在 Windows 系统中，信号处理是通过 Windows API 函数来实现的，这些函数与 POSIX 系统上的信号处理方式不同。在该文件中，OsStorage 和 OsExtraData 这两个结构体是用于将 Windows 信号处理的数据和相关信息进行封装和存储的。

1. OsStorage 结构体：
   - 用于存储在 Windows 系统中，与信号相关的数据，并为信号处理提供数据上下文。
   - 在 struct OsStorage 中，包含了内存中表示 Windows 信号处理器的句柄 (`handle`)。
   - 这个结构体的主要目的是为了在 tokio 应用程序中管理和进行信号处理，向操作系统注册信号处理程序，并在处理信号时提供相应的上下文。

2. OsExtraData 结构体：
   - 表示 Windows 系统中信号处理器的额外信息的结构体。
   - 用于存储附加的信号处理信息。
   - 这个结构体中，主要包含了用于与操作系统进行交互的 Windows 系统调用所必需的数据，如内存中的句柄信息（`handle_data`）等。

这些结构体的主要目的是通过与 Windows API 函数进行交互，将 tokio 应用程序中的信号处理与底层操作系统信号处理机制连接起来，使得 tokio 应用能够在 Windows 系统上进行信号处理。

