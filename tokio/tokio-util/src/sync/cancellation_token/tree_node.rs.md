# File: tokio/tokio-util/src/sync/cancellation_token/tree_node.rs

在tokio-util库的cancellation_token模块中，tree_node.rs文件实现了一个用于取消操作的树结构。该文件定义了两个结构体：TreeNode和Inner。

TreeNode结构体表示树的节点，用于组织和管理取消操作的层次结构。每个TreeNode可以拥有多个子节点和一个父节点，并使用引用计数跟踪其子节点和父节点的数量。TreeNode结构体具有以下字段和方法：

- parent: 表示节点的父节点，是一个可选的Weak引用，用于防止循环引用。
- children: 表示节点的子节点，是一个Vector类型的Arc引用，可以有多个子节点。
- state: 表示节点的状态，是一个AtomicUsize原子类型，用于标识取消状态。
- count: 表示节点的引用计数，是一个AtomicUsize原子类型，用于跟踪子节点和父节点的数量。
- new(): 一个关联函数，用于创建一个新的树节点。
- add_child(): 将一个子节点添加到当前节点的子节点列表中。
- remove_child(): 从当前节点的子节点列表中移除一个子节点。
- is_cancelled(): 检查节点的取消状态是否为已取消。
- set_cancelled(): 将节点的取消状态设置为已取消。

Inner结构体包含了一个树节点，并为其提供了更方便的方法和实现。Inner结构体具有以下字段和方法：

- root: 表示树的根节点，是一个Option类型的Arc引用，可以为空。
- count: 表示内部节点的引用计数，是一个AtomicUsize原子类型，用于跟踪节点的数量。
- new(): 一个关联函数，用于创建一个新的内部节点。
- get_or_insert_root(): 获取或插入树的根节点。
- cancel_all(): 将树的根节点及其所有子节点的取消状态设置为已取消。

概括来说，tree_node.rs文件中的TreeNode和Inner结构体提供了一种用于取消操作的树结构，可以方便地管理和取消相关操作。

