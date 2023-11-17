# File: vector/src/api/server.rs

在Rust生态中的vector项目中，vector/src/api/server.rs文件的作用是定义了与服务器相关的结构体和实现，用于处理来自向量服务器的HTTP请求。

该文件中定义了3个与服务器相关的结构体：ServerError、PrometheusExporter和Server。

- ServerError结构体用于表示服务器错误，其中包含一个message字段用于存储错误消息。

- PrometheusExporter结构体是一个Prometheus指标导出器，用于导出向量的统计信息。

- Server结构体是HTTP服务器，它使用Hyper库来处理HTTP请求，并将请求路由到适当的处理程序。它实现了功能接口，用于处理不同类型的请求。

在Server结构体中，有几个重要的方法：

- new()方法用于创建一个新的Server实例，它初始化了路由映射和Prometheus导出器。

- start()方法用于启动HTTP服务器，并开始监听传入的请求。

- router()方法用于将不同类型的请求映射到相应的处理程序。它使用了Router库来进行路由。

- handle_healthcheck()方法用于处理"/healthcheck"请求，用于检查服务器是否正常运行。

- handle_metrics()方法用于处理"/metrics"请求，用于导出Prometheus指标。

- handle_logs()方法用于处理"/logs"请求，用于查看日志。

- handle_reload()方法用于处理"/reload"请求，用于重新加载配置。

- handle_version()方法用于处理"/version"请求，用于查看向量的版本信息。

- handle_events()方法用于处理"/events"请求，用于发送事件数据。

通过这些方法，Server结构体可以处理不同类型的HTTP请求，并根据请求类型进行相应的操作。它充当了向量服务器的主要组件，负责处理和分发请求，并且通过路由，可以根据请求路径将请求映射到适当的处理程序。

