# File: vector/src/sinks/splunk_hec/metrics/sink.rs

vector/src/sinks/splunk_hec/metrics/sink.rs文件是Rust生态中Vector项目的Splunk HEC（HTTP Event Collector）指标发送器的源代码文件。

该文件中定义了三个struct：HecMetricsSink<S>、EventPartitioner和HecMetricsProcessedEventMetadata。

1. HecMetricsSink<S> struct：这是Splunk HEC指标发送器（Sink）的主要结构体。它实现了Sink trait，负责将Vector收集到的指标数据发送到Splunk HEC接收器。该结构体包含以下主要字段和方法：
   - config: 包含Splunk HEC相关的配置参数，如URL、认证信息等。
   - encoder: 数据的编码器，将数据编码为Splunk HEC要求的格式。
   - http: HTTP客户端，用于与Splunk HEC接收器进行通信。
   - send_stream: 用于异步发送数据的流。
   - event_partitioner: 事件分区器，根据配置将事件分散到不同的线程中处理。
   - processed_events_metadata: 存储已发送的事件的元数据，包括事件ID和发送时间等。用于处理发送成功或失败的回调。

   HecMetricsSink结构体的方法包括：
   - new: 根据配置参数创建一个新的HecMetricsSink实例。
   - is_active: 检查当前是否有活动的发送任务。
   - enqueue: 将事件压入发送队列。
   - send_queued_events: 将队列中的事件发送到Splunk HEC接收器。

2. EventPartitioner struct：事件分区器，根据配置将事件分散到不同的线程中处理。它实现了Partitioner trait。EventPartitioner结构体的主要方法是partition，用于将事件分配到不同的指定线程中处理。

3. HecMetricsProcessedEventMetadata struct：包含已发送事件的元数据，用于处理发送成功或失败的回调。该结构体的主要字段是event_id和sent_at，分别表示事件的唯一标识和发送时间。

这些结构体的作用是为了实现Splunk HEC指标发送器，用于将Vector项目收集到的指标数据发送到Splunk HEC接收器，并记录已发送事件的元数据供后续处理。

