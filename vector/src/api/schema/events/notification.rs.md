# File: vector/src/api/schema/events/notification.rs

在Rust生态vector项目的源代码中，`vector/src/api/schema/events/notification.rs`文件的作用是定义了与事件通知相关的结构体和枚举。

1. `Matched`结构体用于表示匹配类型的事件通知。它包含一个`event`字段，用于存储匹配的事件。

2. `NotMatched`结构体用于表示不匹配类型的事件通知。它包含一个`event`字段，用于存储不匹配的事件。

3. `InvalidMatch`结构体用于表示无效匹配类型的事件通知。它包含一个`error`字段，用于存储无效匹配的错误信息。

4. `EventNotification`结构体用于表示事件通知的类型。它包含一个`event`字段，用于存储通知的具体事件。

5. `Notification`枚举用于表示通知的类型。它可以是`Matched`、`NotMatched`或`InvalidMatch`中的一种，分别表示匹配、不匹配和无效匹配类型的事件通知。

这些结构体和枚举的定义提供了一种灵活的方式来处理不同类型的事件通知，并使得代码易于理解和扩展。通过使用不同的结构体和枚举，可以更好地表示事件通知的不同情况，并且可以根据需要对其进行处理。这样可以更好地支持事件处理逻辑的编写和维护。

