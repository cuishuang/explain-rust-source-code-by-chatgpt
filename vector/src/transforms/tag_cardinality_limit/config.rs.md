# File: vector/src/transforms/tag_cardinality_limit/config.rs

在Rust生态vector项目的源代码中，`vector/src/transforms/tag_cardinality_limit/config.rs`文件的作用是定义了有关标签基数限制的配置结构体和枚举。

首先，该文件定义了名为`TagCardinalityLimitConfig`的结构体。这个结构体包含两个字段：`mode`和`bloom_filter`。其中，`mode`是一个`Mode`枚举类型，用于指定标签基数限制的执行模式。`bloom_filter`是一个`BloomFilterConfig`结构体类型，用于配置布隆过滤器的参数。

接下来，该文件还定义了名为`BloomFilterConfig`的结构体。这个结构体包含两个字段：`false_positive_probability`和`expected_insertions`。`false_positive_probability`是一个浮点数，用于指定允许的误判率。`expected_insertions`是一个非负整数，用于指定布隆过滤器的期望插入量。

此外，该文件还定义了两个枚举类型：`Mode`和`LimitExceededAction`。

`Mode`枚举类型用于指定标签基数限制的执行模式，共有三个取值：

1. `Passthrough`: 表示标签基数限制不生效，即不对标签进行限制，所有数据都通过。
2. `TrackAndEnforce`: 表示标签基数限制生效，并跟踪标签基数，如果超过限制则执行指定的动作。
3. `Apply`: 表示标签基数限制生效，并将超出限制的数据标记为无效。

`LimitExceededAction`枚举类型用于指定当标签基数超过限制时的动作，共有三个取值：

1. `RejectMessage`: 表示丢弃整个消息。
2. `RejectEvent`: 表示丢弃事件。
3. `LogError`: 表示记录错误日志，但仍然允许数据继续处理。

以上就是`vector/src/transforms/tag_cardinality_limit/config.rs`文件中定义的`TagCardinalityLimitConfig`、`BloomFilterConfig`结构体和`Mode`、`LimitExceededAction`枚举的作用和对应字段的含义。它们被使用在vector项目的标签基数限制功能中，用于配置和控制标签基数的限制和处理方式。

