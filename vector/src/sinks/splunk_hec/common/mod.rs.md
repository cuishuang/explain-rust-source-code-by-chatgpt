# File: vector/src/sinks/splunk_hec/common/mod.rs

在Rust生态vector项目中，`vector/src/sinks/splunk_hec/common/mod.rs`文件的作用是定义了与Splunk HTTP Event Collector（HEC）相关的公共实现。

这个文件中定义的结构体和枚举用于在发送数据至Splunk HEC时进行配置和处理。其中，`EndpointTarget`这个枚举定义了三种可能的端点目标类型：

1. `Hec` - 代表将事件发送到Splunk HEC的HTTP地址。
2. `HostAndPort` - 代表将事件发送到某个主机和端口号。
3. `HostAndScheme` - 代表将事件发送到某个主机和特定的传输协议。

这些不同的目标类型可以根据用户在配置文件中提供的参数值来选择。

此外，还定义了一些与将事件发送至Splunk HEC相关的结构体和函数。例如，`SplunkHecConfigBuilder`结构体用于构建Splunk HEC配置，包括端点目标、验证方法和认证信息等。`HecSink`结构体用于实际将事件发送至Splunk HEC，并处理相应的响应。

总体而言，`vector/src/sinks/splunk_hec/common/mod.rs`文件的作用是提供了一套通用的功能和实现，用于将事件发送至Splunk HEC，并对配置进行处理和验证。

