# File: rust-analyzer/crates/ide/src/extend_selection.rs

在rust-analyzer中的"extend_selection.rs"文件是实现了扩展选中功能的模块。这个功能允许在IDE中通过连续按下相同快捷键来扩展当前选中的代码范围。

该文件中定义了几个结构体，包括：`Selection`，`SelectionRange`和`TextRange`，它们分别用于处理选中的文本范围。

1. `Selection`结构体表示当前选中的文本范围，包括`anchor`和`head`两个属性，分别代表选中区间的起点和终点。

2. `SelectionRange`结构体代表了扩展选中的结果。它包含了一个`range`属性，用于表示扩展选中的文本范围，以及一个`parent`属性，指向当前范围的父级选中。

3. `TextRange`结构体用于表示文本的范围，包括`start`和`end`属性，分别表示文本范围的起点和终点。

这些结构体的组合和逻辑在整个文件中被用来实现扩展选中功能的具体逻辑。具体来说，当用户按下扩展选中的快捷键时，该模块会根据当前光标所在的位置和上次扩展选中的结果，计算出新的选中范围，并返回给IDE进行相应操作，例如高亮显示、代码补全等。

总结来说，"extend_selection.rs"文件中的结构体和函数提供了扩展选中功能的核心实现，使得rust-analyzer能够更好地支持代码编辑的便捷性和高效性。

