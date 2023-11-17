# File: vector/src/config/graph.rs

在Rust生态vector项目中,vector/src/config/graph.rs文件的作用是定义图形配置和图形结构。

Graph模块中定义的Edge结构体表示图形中的边。每个边包含一个源节点（source）和目标节点（target），可以使用Edge::add_edge函数向图形中添加边。

Graph模块中的Graph结构体表示整个图形，它包含一个节点列表和一个边列表。Graph结构体实现了向图中添加节点、添加边，以及从图中获取节点和边的方法。Graph结构体还定义了一些辅助方法，如获取入边和出边的数量。

Node枚举定义了图形中的节点类型。它有多个变体，每个变体表示不同类型的节点。例如，Node::Input表示输入节点，Node::Transform表示转换节点，Node::Output表示输出节点等。每个Node变体包含该节点的唯一标识符（id）和边的列表。Node枚举实现了一些用于添加边和节点的方法。

总而言之，vector/src/config/graph.rs文件定义了用于构建图形配置和处理图形结构的数据结构，包括节点、边以及图形本身。它提供了一些方法来操作和查询图形中的节点和边。

