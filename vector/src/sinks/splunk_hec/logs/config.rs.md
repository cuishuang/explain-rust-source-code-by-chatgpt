# File: vector/src/sinks/splunk_hec/logs/config.rs

在Rust生态"vector"项目的源代码中，"vector/src/sinks/splunk_hec/logs/config.rs"文件的作用是用于定义和配置将数据发送到Splunk HEC（HTTP Event Collector）的日志记录器 sink 的相关配置。

该文件中包含了以下几个结构体对配置进行了定义和描述:

1. `HecLogsSinkConfig`：这个结构体是整个配置文件的入口，它代表了Splunk HEC日志记录器的配置选项。主要包含以下字段：

   - `host`: 指定要发送到的Splunk HEC的主机地址。
   - `port`: 指定要发送到的Splunk HEC的端口号。
   - `token`: 指定要发送到的Splunk HEC的认证令牌。
   - `batch_size`: 指定每个批量发送的事件数量。
   - `timeout_secs`: 指定发送超时的时间限制。
   - `tls`: 指定是否启用TLS/SSL加密传输。
   - `encoding`: 指定事件编码类型（JSON或Text）。
   - `injection`: 指定是否启用字段注入功能。
   - `message_key`: 指定事件中的字段作为消息文本的键。

2. `TlsConfig`：这个结构体用于配置TLS/SSL设置，包含以下字段：

   - `enabled`: 指定是否启用TLS/SSL加密传输。
   - `ca_path`: 指定CA证书的路径。
   - `ca_file`: 指定CA证书的文件名。
   - `cert_path`: 指定客户端证书的路径。
   - `cert_file`: 指定客户端证书的文件名。
   - `key_path`: 指定客户端证书的私钥路径。
   - `key_file`: 指定客户端证书的私钥文件名。
   - `key_pass`: 指定客户端证书的私钥密码。

3. `EncodingConfig`：这个结构体用于配置事件的编码类型，包含以下字段：

   - `enabled`: 指定是否启用事件编码。
   - `json`: 指定是否使用JSON编码。
   - `message_key`: 指定事件中使用的字段键。

4. `InjectionConfig`：这个结构体用于配置字段注入功能，包含以下字段：

   - `enabled`: 指定是否启用字段注入。
   - `fields`: 指定要注入到事件中的字段和值的映射。

这些结构体的字段提供了对Splunk HEC日志记录器的各种配置选项的灵活性和可定制性。通过修改和组合这些配置选项，可以根据需求来配置和使用Splunk HEC日志记录器进行事件的发送和注入。

