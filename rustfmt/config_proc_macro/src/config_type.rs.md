# File: /Users/fliter/rust-contribute/rustfmt/config_proc_macro/src/config_type.rs

在Rust的rustfmt项目中，/Users/fliter/rust-contribute/rustfmt/config_proc_macro/src/config_type.rs文件定义了一些与配置相关的数据结构。该文件的主要作用是实现rustfmt的配置参数以及对应的解析逻辑。

在该文件中，有几个enum类型，分别是：

1. `IndentStyle`: 该枚举定义了代码块缩进的样式选项，包括`Block`，`Visual`和`Layered`。`Block`表示使用块缩进风格，即使用固定数量的空格缩进每一层代码块；`Visual`表示使用可视化缩进风格，按照代码的视觉对齐进行缩进；`Layered`表示使用分层缩进风格，根据代码的层次结构进行缩进。

2. `WidthHeuristics`: 该枚举定义了代码行的宽度启发式选项，包括`MaxWidth`和`Percent`. `MaxWidth`表示最大宽度启发式选项，可以设置代码行的最大宽度，超过该宽度的代码将被换行；`Percent`表示百分比启发式选项，可以设置代码行宽度相对于屏幕宽度的百分比。

3. `UseSmallIntegerFormatting`: 该枚举定义了小整数格式化选项，包括`Never`、`Always`和`FallBack`. `Never`表示不对小整数进行特殊格式化处理；`Always`表示始终对小整数进行特殊格式化处理；`FallBack`表示在特定情况下对小整数进行特殊格式化处理，比如当它们处在包含混合语句的行中时。

4. `ReorderFields`: 该枚举定义了字段重新排序选项，包括`None`、`Named`和`Sorted`. `None`表示不重新排序字段；`Named`表示按照字段的定义顺序重新排序字段；`Sorted`表示按照字段名称的字母顺序重新排序字段。

这些enum类型提供了一些配置选项，用户可以根据自己的需求选择适合的选项，以定制rustfmt的代码格式化。在rustfmt项目的其他代码中，这些enum类型将被用于分析用户配置参数，确定代码格式化的方式和规则。

