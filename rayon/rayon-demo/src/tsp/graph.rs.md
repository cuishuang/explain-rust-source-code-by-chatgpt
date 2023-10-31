# File: rayon/rayon-demo/src/tsp/graph.rs

在Rust的rayon-demo项目中，rayon/rayon-demo/src/tsp/graph.rs文件定义了旅行商问题（Traveling Salesman Problem，TSP）的图数据结构。

Graph是整个TSP图的结构体，包含了所有的节点和边。它的作用是提供一个表示TSP问题的图的数据结构，以及相关的操作方法。

Node是图中的节点结构体，包含了节点的id和对应的坐标（x和y）。它的作用是表示图中的一个具体节点。

Edge是图中的边结构体，包含了起始节点和目标节点的id，以及这条边的长度。它的作用是表示图中的一条具体边。

NodeSet是节点集合，是一个包含节点的集合的结构体。它的作用是为了方便节点的管理和操作，提供了一些集合操作的方法。

这些结构体的作用是构建和表示TSP的图结构，其中Graph是整个图的容器，包含了所有的节点和边；Node表示图中的一个节点；Edge表示图中的一条边；NodeSet则是用于存储节点的集合。通过这些结构体，可以方便地进行TSP图的构建、操作和计算。

