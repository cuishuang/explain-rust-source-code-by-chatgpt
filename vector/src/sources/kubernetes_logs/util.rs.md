# File: vector/src/sources/kubernetes_logs/util.rs

vector/src/sources/kubernetes_logs/util.rs文件是Rust生态vector项目中的一个文件，主要提供了用于处理Kubernetes日志的实用工具函数和结构体。

1. 函数`build_config_from_args`用于从命令行参数构建Kubernetes配置对象。它获取命令行参数中的kubeconfig文件路径、namespace、container等参数，并构建一个`Config`结构体对象，该对象表示Kubernetes的配置信息。

2. 函数`parse_labels`用于解析传入的标签字符串，并返回一个`HashMap`对象，其中包含了解析得到的标签。

3. 结构体`KubernetesLogsArgs`定义了用于表示Kubernetes日志参数的结构体。它包含了用于命令行解析时使用的字段，例如kubeconfig文件路径、namespace、container、标签等。

4. 结构体`KubernetesLogsSource`是`Source` trait的一个实现，表示从Kubernetes日志源采集日志。它包含了一些字段，例如kubeconfig的`Config`对象、namespace、container、标签等，用于表示从Kubernetes中采集日志时的相关配置。

5. 结构体`KubernetesPodLogsSource`是`KubernetesLogsSource`的一个具体实现，在采集日志时会监听Kubernetes的API服务器，并根据配置的namespace、container和标签来过滤日志。它实现了`Source` trait的方法，用于启动和关闭日志采集。

总的来说，vector/src/sources/kubernetes_logs/util.rs文件提供了一些用于处理Kubernetes日志的实用函数和结构体，可以方便地构建Kubernetes配置、解析标签、以及采集Kubernetes日志。

