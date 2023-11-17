# File: vector/src/sources/host_metrics/cgroups.rs

文件cgroups.rs定义了与CGroups（CGroup）相关的功能，并提供了用于监视和获取主机度量指标的实现。

具体的结构体和枚举的作用如下：

1. CGroupRecurser<'a>：一个递归遍历CGroup的结构体，它使用迭代器模式递归遍历CGroup层次结构，并提供了一些方法用于检查和遍历CGroup的子组。

2. CGroupRoot：表示CGroup层次结构中的根节点，它包含了CGroup的路径和一些方法用于操作和管理CGroup。

3. CGroup：表示CGroup层次结构中的一个节点，它包含了CGroup的路径、名称和一些方法用于操作和管理CGroup。

4. Setup (TempDir)：表示CGroup的设置，主要用于设置临时目录。

这些结构体主要用于定义CGroup相关的功能和操作。

而枚举类型的作用如下：

1. CGroupsError：定义了可能发生的CGroup相关的错误，例如无法找到CGroup、读取CGroup内容失败等。

2. Mode：定义了CGroup的模式，包括当前CGroup是否可读、是否可写等。

