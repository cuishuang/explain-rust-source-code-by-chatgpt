# File: vector/src/sinks/new_relic/config.rs

vector/src/sinks/new_relic/config.rs文件在Rust生态中的vector项目中是用来配置New Relic Sink的文件。该文件定义了一些用于配置New Relic Sink的结构体和枚举。

1. NewRelicDefaultBatchSettings结构体：它用于配置批量发送数据到New Relic的默认设置，包括批处理最大大小、最大等待时间、重试次数等。

2. NewRelicApiRetry结构体：它用于配置在API请求失败时的重试策略，包括初始退避时间、最大退避时间、最大重试次数等。

3. NewRelicConfig结构体：它是整个New Relic Sink的配置结构体，包括New Relic区域、API密钥、批处理设置等。通过该结构体可以配置Sink将数据发送到指定的New Relic实例。

4. NewRelicCredentials结构体：它用于配置New Relic的API凭证，包括API密钥等信息。

这些结构体定义了New Relic Sink的各种配置选项，用户可以根据自己的需求对这些选项进行配置，以满足不同的使用场景。

此外，该文件还定义了以下两个枚举：

1. NewRelicRegion枚举：它定义了New Relic的区域选项，包括美国、欧洲和默认等选项。用户可以根据需要选择适合自己的区域。

2. NewRelicApi枚举：它定义了New Relic的API选项，包括发送指标数据和发送日志数据等选项。用户可以根据需求选择适合自己的API。

通过这些枚举，用户可以在配置New Relic Sink时选择合适的区域和API类型，以便满足具体的数据发送需求。

