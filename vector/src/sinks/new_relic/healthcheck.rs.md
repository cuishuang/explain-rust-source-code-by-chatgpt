# File: vector/src/sinks/new_relic/healthcheck.rs

在Rust生态中的vector项目中，`vector/src/sinks/new_relic/healthcheck.rs`文件是用于实现与New Relic健康检查相关的功能。它提供了三个结构体`NewRelicStatusModel`、`NewRelicStatusPage`和`NewRelicStatusComponent`。

1. `NewRelicStatusModel` 结构体代表整个New Relic健康状态的模型。它包含了一组`NewRelicStatusPage`对象，每个对象代表New Relic健康状态页面的不同部分。`NewRelicStatusModel`主要用于将整个健康状态的模型序列化为JSON格式，并返回给用户。

2. `NewRelicStatusPage` 结构体代表New Relic健康状态页面的不同部分。每个页面都可以包含多个`NewRelicStatusComponent`对象，表示该页面中的不同组件。`NewRelicStatusPage`用于将页面的不同组件序列化为JSON格式，并返回给用户。

3. `NewRelicStatusComponent` 结构体代表New Relic健康状态页面中的一个组件。每个组件包含了一些指标，例如组件的名称、状态、错误信息等。`NewRelicStatusComponent`用于将组件的指标信息序列化为JSON格式，并返回给用户。

这些结构体提供了一种将New Relic健康状态数据组织成层级结构，并将其序列化为JSON格式的方式。这样通过访问相应的API，可以获取包含整个向量实例健康状态的JSON数据，并进行监控和分析。

