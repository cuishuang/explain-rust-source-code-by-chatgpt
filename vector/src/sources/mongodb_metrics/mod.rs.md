# File: vector/src/sources/mongodb_metrics/mod.rs

文件 `vector/src/sources/mongodb_metrics/mod.rs` 的作用是实现了涉及 MongoDB 数据库的指标收集器，用于从 MongoDB 数据库中收集各种度量指标。

具体来说，该文件定义了以下内容：

**Structs:**

1. `MongoDbMetricsConfig`: 这个结构体用于定义配置参数，通过读取用户提供的配置文件来构建。包括 MongoDB 服务器的主机和端口，认证凭据，以及要收集的度量指标的名称和采样间隔等。

2. `MongoDbMetrics`: 这个结构体是 MongoDB 指标收集器的主要实现。它包含一个 MongoDB 的连接池，用于与数据库建立连接和查询指标数据。`MongoDbMetrics` 使用 `tokio` 库来进行异步操作，可以周期性地查询 MongoDB 服务器上的度量指标。

**Enums:**

1. `BuildError`: 这个枚举类型包含了在构建 MongoDB 指标收集器时可能发生的错误情况。例如，无法建立与 MongoDB 的连接，无法读取配置文件等。

2. `CollectError`: 这个枚举类型包含了在收集 MongoDB 指标时可能出现的错误情况。例如，执行查询时发生错误，无法解析返回的结果等。

总的来说，`MongoDbMetricsConfig` 结构体用于配置 MongoDB 指标收集器的参数，`MongoDbMetrics` 结构体实现了实际的指标收集过程。`BuildError` 和 `CollectError` 枚举则分别表示在构建和收集指标过程中可能出现的错误情况。

