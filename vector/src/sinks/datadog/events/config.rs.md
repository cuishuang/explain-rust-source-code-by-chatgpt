# File: vector/src/sinks/datadog/events/config.rs

在Rust生态的vector项目中，`vector/src/sinks/datadog/events/config.rs`文件的作用是定义Datadog事件日志的配置。

该文件中定义了三个相关的结构体：DatadogEventsConfig，MappedFieldType，和EventTypeTransform。

1. `DatadogEventsConfig`结构体用于描述Datadog事件日志的配置选项。它包含以下字段：

   - `api_key`：Datadog API密钥，用于身份验证。
   - `site`：Datadog网站的地址，用于指定Datadog云服务的区域位置（默认为`datadoghq.com`）。
   - `service`：Datadog事件日志所属的服务名称。
   - `source_type_name`：事件日志的源类型名称。
   - `source_category`：事件日志的源分类。
   - `namespace`：事件日志的命名空间。
   - `tags`：关联到所有事件日志的标签集合。

2. `MappedFieldType`是根据Datadog API支持的字段类型定义的枚举。目前支持的类型包括：

   - `String`：字符串类型。
   - `Integer`：整数类型。
   - `Float`：浮点数类型。
   - `Boolean`：布尔类型。
   - `Datetime`：日期时间类型。

3. `EventTypeTransform`结构体用于定义将事件类型（Event Type）映射到特定字段的转换规则。它包含以下字段：

   - `from_field`：转换的源字段名。
   - `to_field`：转换的目标字段名。
   - `field_type`：目标字段的类型。
   - `transforms`：关联到该转换的转换器集合。每个转换器定义一个数据处理规则，如字符串替换、正则表达式替换等。

`DatadogEventsConfig`结构体允许用户配置Datadog事件日志的各种选项，例如API密钥、服务名称、标签、字段类型等。`MappedFieldType`枚举定义可用的字段类型，而`EventTypeTransform`结构体用于定义事件类型到字段的转换规则。这些结构体的使用可以在实例化和配置Datadog事件日志的时候提供便利，并确保数据的正确处理和发送。

