# File: vector/src/topology/running.rs

在Rust生态vector项目的源代码中，vector/src/topology/running.rs文件的作用是定义和实现了运行时拓扑。

详细介绍一下该文件内容如下：

1. RunningTopology结构体：它是整个运行时拓扑的主要结构体，用于在运行时管理和控制各个组件之间的连接和通信。它包含了以下字段：

- data_plane: DataPlane实例，用于处理与数据平面相关的任务，如消息的接收、发送和路由。
- ctrl_plane: CtrlPlane实例，用于处理与控制平面相关的任务，如组件的注册、脱机等。
- healthchecks: Healthchecks实例，用于管理和监视各个组件的健康状态。
- metrics: Metrics实例，用于收集和展示各个组件的度量信息。
- outbound_observers: 用于传输出站观察者。
- teardown_handle: 用于协调析构顺序的句柄。
- terminated: bool值，表示是否已终止运行。

2. DataPlane结构体：它是数据平面的主要结构体，用于处理与数据的接收、发送和路由相关的任务。它包含了以下字段：

- healthchecks: Healthchecks实例，用于管理和监视数据平面的健康状态。
- managed_elasticsearch_config: Option<Value>类型，表示管理的Elasticsearch配置信息。

3. CtrlPlane结构体：它是控制平面的主要结构体，用于处理与组件注册、脱机等相关的任务。它包含了以下字段：

- healthchecks: Healthchecks实例，用于管理和监视控制平面的健康状态。

4. Healthchecks结构体：它是用于管理和监视各个组件的健康状态的结构体。它包含了以下字段：

- hc_unhealthy: Vec<String>类型，表示健康状态不健康的组件列表。

5. Metrics结构体：它是用于收集和展示各个组件的度量信息的结构体。

另外，上述结构体还实现了一些方法和trait，用于处理和管理各个组件之间的连接、通信和状态。

