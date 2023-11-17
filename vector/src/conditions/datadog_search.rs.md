# File: vector/src/conditions/datadog_search.rs

文件`vector/src/conditions/datadog_search.rs`的作用是实现与Datadog搜索相关的功能。具体来说，该文件定义了三个结构体：`DatadogSearchConfig`、`DatadogSearchRunner`和`EventFilter`。

`DatadogSearchConfig`是一个配置结构体，用于指定在Datadog中执行搜索所需的参数，包括Datadog的API密钥、应用密钥、搜索字符串、时间范围等。它的作用是允许用户对搜索进行自定义配置。

`DatadogSearchRunner`是一个运行器结构体，用于执行Datadog搜索并返回结果。它封装了与Datadog进行交互的逻辑，包括构建API请求、发送请求、解析响应等。它的作用是实际执行搜索操作。

`EventFilter`是一个事件过滤器结构体，用于过滤从Datadog搜索返回的事件结果。它定义了一些过滤条件，如事件名称、优先级等，并提供了方法用于检查一个事件是否匹配这些条件。它的作用是允许用户根据自定义规则对搜索结果进行过滤。

这些结构体的目的是为了在Vector项目中实现与Datadog的集成。通过使用这些结构体，用户可以配置Vector来执行Datadog搜索并将结果传递给下一个处理步骤，同时还可以根据自定义规则过滤搜索结果。

