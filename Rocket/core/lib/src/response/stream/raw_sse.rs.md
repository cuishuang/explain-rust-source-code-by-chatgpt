# File: Rocket/core/lib/src/response/stream/raw_sse.rs

在Rocket web框架的源代码中，`raw_sse.rs`文件位于`Rocket/core/lib/src/response/stream`目录下，它实现了用于处理服务器发送事件（Server Sent Event，简称SSE）的原始流。

SSE是一种在客户端和服务器之间实时发送数据的通信协议。`raw_sse.rs`文件中定义了用于构建和发送SSE的数据类型和函数。

文件中的主要数据类型是`RawLinedEvent`和`State`。

1. `RawLinedEvent`结构体用于表示SSE中的每个事件。它包含事件的标题（title）和数据（data）。`RawLinedEvent`结构体的定义如下：

```rust
struct RawLinedEvent {
    title: Option<String>,
    data: String,
}
```

- `title`字段是一个可选的字符串，表示事件的标题。
- `data`字段是一个字符串，表示事件的数据。

2. `State`枚举类型用于表示SSE的状态。它定义了三个可能的状态：`Loading`, `Active`和`Complete`。

- `Loading`状态表示SSE正在加载中。
- `Active`状态表示SSE处于活动状态，即正在传输数据。
- `Complete`状态表示SSE已完成传输。

`State`枚举的定义如下：

```rust
enum State {
    Loading,
    Active,
    Complete,
}
```

`raw_sse.rs`文件中还实现了处理SSE传输的相关函数，包括：

- `write_event()`函数：用于将事件数据写入到底层的写缓冲区中。
- `flush()`函数：用于刷新底层的写缓冲区，将数据发送给客户端。
- `set_loading()`, `set_active()`和`set_complete()`函数：用于设置SSE的状态。

通过使用`raw_sse.rs`文件中定义的数据类型和函数，Rocket框架能够提供对SSE的支持，使开发者能够方便地构建实时数据传输的应用程序。

