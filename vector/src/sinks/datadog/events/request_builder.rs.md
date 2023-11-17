# File: vector/src/sinks/datadog/events/request_builder.rs

vector/src/sinks/datadog/events/request_builder.rs文件在Rust生态的Vector项目中负责构建Datadog的事件请求。

1. DatadogEventsRequest是向Datadog发送事件请求的结构体。它包含了需要发送的事件数据，例如事件标题、事件内容、标签等。通过实例化DatadogEventsRequest结构体，我们可以向Datadog发送事件请求。

2. Metadata结构体用于表示事件的元数据，包括事件的来源、主机、优先级等信息。它提供了一系列方法来设置和获取元数据。

3. DatadogEventsRequestBuilder是一个构建者（Builder）模式的辅助结构体，用于简化构建DatadogEventsRequest对象的过程。它允许开发者逐步设置事件的各个属性，并最终构建出一个完整的DatadogEventsRequest对象。

具体来说，DatadogEventsRequestBuilder提供了以下方法来设置事件的各个属性：

- `add_event`：向请求中添加一个事件。
- `add_events`：向请求中添加多个事件。
- `with_metadata`：设置事件的元数据。
- `with_priority`：设置事件的优先级。
- `build`：构建DatadogEventsRequest对象。

通过使用DatadogEventsRequestBuilder，开发者可以方便地按需设置事件的各个属性，然后通过调用build方法构建出一个完整的DatadogEventsRequest对象，用于向Datadog发送事件请求。

总结起来，vector/src/sinks/datadog/events/request_builder.rs文件负责构建Datadog事件请求，并提供了DatadogEventsRequest、Metadata和DatadogEventsRequestBuilder三个结构体，分别用于表示事件请求、事件元数据以及构建事件请求对象。这样做的目的是为了提供一个便捷的方式来构建和发送Datadog的事件请求。

