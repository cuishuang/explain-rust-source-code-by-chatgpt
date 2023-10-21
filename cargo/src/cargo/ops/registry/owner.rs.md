# File: cargo/src/cargo/ops/registry/owner.rs

在Rust Cargo的源代码中，cargo/src/cargo/ops/registry/owner.rs文件的作用是处理与包所有者相关的操作，包括添加、删除和列出包所有者。

该文件实现了以下功能：

1. `add_owners()`函数：用于向包中添加新的所有者，接受仓库地址、包名和所有者列表作为参数，通过调用注册表API将所有者添加到包中。

2. `remove_owners()`函数：用于从包中移除现有的所有者，接受仓库地址、包名和所有者列表作为参数，通过调用注册表API将所有者从包中删除。

3. `list_owners()`函数：用于列出包的所有者，接受仓库地址和包名作为参数，通过调用注册表API获取所有者列表并返回。

此外，还定义了一些与所有者操作相关的结构体，包括：

1. `AddOwnersOptions`：添加所有者选项。其字段为`index`, `token`和`owners`，分别表示注册表索引地址、访问令牌和要添加的所有者列表。

2. `RemoveOwnersOptions`：移除所有者选项。其字段为`index`, `token`和`owners`，分别表示注册表索引地址、访问令牌和要移除的所有者列表。

3. `ListOwnersOptions`：列出所有者选项。其字段为`index`, `token`和`package`，分别表示注册表索引地址、访问令牌和要列出所有者的包名。

这些结构体用于传递相关的参数信息，并在执行所有者操作时提供必要的信息和上下文。

