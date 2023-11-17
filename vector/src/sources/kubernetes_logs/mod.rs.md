# File: vector/src/sources/kubernetes_logs/mod.rs

在Rust生态中的vector项目中，vector/src/sources/kubernetes_logs/mod.rs文件提供了一种用于从Kubernetes集群中获取日志的源数据的实现。它是vector日志收集器的一部分，负责从Kubernetes的实时日志API中抓取日志。

该文件中定义了两个struct：Config和Source。

1. Config:
   - Config结构体定义了用于配置Kubernetes Logs源的参数和选项。它有以下字段:
     - `in_cluster`: 这是一个布尔值，表示是否在集群内运行。如果设置为true，vector将使用Kubernetes的内部API服务进行访问，否则将使用配置文件中提供的kubeconfig文件。
     - `namespace`: 这是一个可选的字符串，表示要从哪个Kubernetes命名空间中获取日志。如果未指定，则默认为所有命名空间。
     - `pod`: 这是一个可选的字符串，表示要从哪个Kubernetes Pod中获取日志。如果未指定，则默认为所有Pod。
     - `container`: 这是一个可选的字符串，表示要从哪个Kubernetes容器中获取日志。如果未指定，则默认为所有容器。
     - `exclude`: 这是一个可选的字符串，表示要排除的日志来源。可以使用正则表达式进行匹配，如果匹配成功，则不会收集该日志。
     - `log_namespace_labels`: 这是一个可选的字符串切片，表示要从哪些Kubernetes命名空间中提取标签。这些标签将作为字段添加到每个日志事件中。默认情况下，不提取任何标签。

2. Source:
   - Source结构体是实现Kubernetes Logs源的主要逻辑。它实现了`Source` trait，为数据源提供了必要的方法。
   - 它的`run`方法负责使用Kubernetes API客户端建立长连接，并根据配置的参数不断获取Kubernetes日志。获取到的日志将被格式化为LogEvent，然后通过推送到vector的记录队列中。
   - `stop`方法用于停止源数据的获取，提交了stop信号，以便退出run方法中的循环并关闭与Kubernetes API的连接。

总之，vector/src/sources/kubernetes_logs/mod.rs文件中的Config和Source结构体提供了从Kubernetes集群中获取日志的配置和实现逻辑，可以方便地集成Kubernetes日志收集功能到vector日志收集器中。

