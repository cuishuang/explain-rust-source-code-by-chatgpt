# File: vector/src/graph.rs

在Rust生态vector项目的源代码中，`vector/src/graph.rs` 文件的作用是实现了一个通用的无向图数据结构，用于维护节点之间的关系。

该文件中定义了一个名为 `Graph` 的结构体，表示整个图。`Graph` 结构体包含以下字段：
- `nodes: Vec<Node>`：一个 `Node` 类型的动态数组，用于存储图中的所有节点。
- `edges: Vec<Edge>`：一个 `Edge` 类型的动态数组，用于存储图中的所有边。

`Node` 结构体表示图中的节点，它具有以下字段：
- `name: String`：节点的名称，以字符串表示。
- `metadata: NodeMetadata`：存储节点的一些额外元数据的结构体。
- `edges: Vec<usize>`：存储与该节点相连的边的索引数组。

`Edge` 结构体表示图中的边，它具有以下字段：
- `source: usize`：边的起始节点在 `Graph` 的 `nodes` 动态数组中的索引。
- `destination: usize`：边的结束节点在 `Graph` 的 `nodes` 动态数组中的索引。
- `metadata: EdgeMetadata`：存储边的一些额外元数据的结构体。

此外，`graph.rs` 文件中还定义了与图相关的函数和方法，用于对图进行不同的操作。这些函数和方法包括：
- `new()`：创建一个新的空图。
- `add_node(&mut self, name: String, metadata: NodeMetadata)`：向图中添加一个新的节点。
- `add_edge(&mut self, source: usize, destination: usize, metadata: EdgeMetadata)`：向图中添加一条新的边。
- `get_node(&self, name: &str) -> Option<usize>`：根据节点名称查找节点在 `nodes` 动态数组中的索引。
- `get_edge(&self, source: usize, destination: usize) -> Option<usize>`：根据起始节点和结束节点查找边在 `edges` 动态数组中的索引。

`Opts` 结构体是一个命令行选项的配置结构体，用于解析和存储命令行参数。该结构体的字段与命令行参数一一对应，包括：
- `subcommand: Subcommand`：用于表示不同子命令的枚举类型。
- `flags: Vec<Flag>`：用于表示命令行的标志选项的动态数组，每个 `Flag` 结构体表示一个标志选项。
- `interval: Option<u64>`：用于表示一个可选的间隔时间，以毫秒为单位。
- `filename: Option<PathBuf>`：用于表示一个可选的文件路径。

