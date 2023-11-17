# File: vector/src/sources/kubernetes_logs/parser/cri.rs

文件cri.rs的作用是解析Kubernetes容器运行时接口(CRI)的日志数据。

具体而言，该文件定义了两个struct：Cri和ParsedLog<'a>。

- Cri struct：表示CRI的日志来源和相关信息。它包含以下字段：
  - `container_id`: 容器的唯一标识符。
  - `pod_uid`: Pod的唯一标识符。
  - `namespace`: 容器所属的命名空间。
  - `container_name`: 容器的名称。
  - `log_data`: 原始日志数据。

- ParsedLog<'a> struct：表示解析后的CRI日志。它包含以下字段：
  - `container_id`: 容器的唯一标识符。
  - `pod_uid`: Pod的唯一标识符。
  - `namespace`: 容器所属的命名空间。
  - `container_name`: 容器的名称。
  - `log_lines`: 解析后的日志行。

在Cri struct中，log_data字段包含原始的日志数据。Cri struct的parse方法将原始数据解析为ParsedLog<'a> struct。ParsedLog<'a> struct的log_lines字段是解析后的日志行的集合。

cri.rs文件提供了CRI日志的解析功能，使得在Rust生态vector项目中可以方便地处理和分析Kubernetes容器的日志数据。

