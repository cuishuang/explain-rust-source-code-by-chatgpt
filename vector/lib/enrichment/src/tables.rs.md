# File: vector/lib/enrichment/src/tables.rs

在Rust生态的vector项目中，`tables.rs`文件位于`vector/lib/enrichment/src/`目录中。该文件的作用是定义了与表格相关的结构体和函数。

`TableRegistry`是一个结构体，用于注册和管理不同表格的实例。它具有以下几个重要的成员函数和属性：

- `new()`: 创建一个空的`TableRegistry`实例。
- `register`: 注册一个表格实例，将其存储在内部的表格映射(`TableMap`)中。
- `get`: 根据表格名称从内部的表格映射中获取对应的表格实例。
- `lock`: 获得表格映射(`TableMap`)的读锁，以允许并发访问。
- `tables`: 返回一个迭代器，用于遍历所有已注册的表格实例。

---
`TableSearch`是一个结构体，保存了一个原子引用计数智能指针(`Arc`)，其中包含另一个原子引用计数智能指针(`ArcSwap`)，该智能指针存储了一个可选的`TableMap`实例。这个结构体的作用是提供一个线程安全的方式来共享和更新`TableMap`实例。

- `TableSearch`的主要作用是允许在多个线程之间安全地访问和共享`TableRegistry`中的表格映射(`TableMap`)。
- `ArcSwap`是一个原子交换结构，它允许原子地交换一个智能指针，并且可以在读取时保留其生命周期。
- `Option<TableMap>`表示`TableMap`是一个可选的值，因此可以为空。

综上所述，`tables.rs`文件中的`TableRegistry`结构体用于注册和管理表格实例，而`TableSearch`结构体用于安全地共享和更新表格映射。这两个结构体在向量项目中扮演了重要的角色，使得在并发环境下对表格进行操作变得更加安全和有效。

