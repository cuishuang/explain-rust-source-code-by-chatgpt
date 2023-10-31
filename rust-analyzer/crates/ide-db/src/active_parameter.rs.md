# File: rust-analyzer/crates/ide-db/src/active_parameter.rs

源代码文件rust-analyzer/crates/ide-db/src/active_parameter.rs的作用是支持自动补全功能中的活动参数（Active Parameter）功能。Active Parameter是指在函数调用时，通过插入符（caret）的位置，来自动弹出参数提示并高亮活动参数的功能。

该文件中定义了几个结构体（Struct）来实现Active Parameter功能：

1. `ActiveParameterInfo`：定义了活动参数的信息，包括参数的索引、开始位置和结束位置。它用于跟踪当前活动参数的信息。

2. `LinkedActiveParameterInfo`：定义了关联活动参数的信息，包括参数的索引和在源代码中的位置。它用于辅助实现关联活动参数的跳转功能。

3. `ActiveParameter`：定义了Active Parameter的主要功能。它包含了一个栈（Stack）用于存储`ActiveParameterInfo`对象，并提供了一系列方法用于处理活动参数的推入和弹出。同时，它还提供了方法来获取当前活动参数信息、以及根据活动参数修改插入符的位置等。

通过这几个结构体的协作，Active Parameter功能能够在编辑器中实时显示函数参数的提示，在函数调用时高亮显示活动参数，并支持根据活动参数进行快速跳转。这有助于提高开发人员的代码编写速度和准确性。

