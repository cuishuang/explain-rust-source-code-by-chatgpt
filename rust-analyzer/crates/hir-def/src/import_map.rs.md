# File: rust-analyzer/crates/hir-def/src/import_map.rs

在rust-analyzer的源代码中， `import_map.rs` 文件的作用是处理代码中的导入语句（import statements），并维护一个从导入项到相应定义（definition）的映射关系。以下是对该文件中相关结构体（struct）、特质（trait）和枚举（enum）的详细介绍：

### 结构体（struct）：

1. `ImportInfo`：保存导入项的详细信息，包括导入路径、所在模块等。

2. `ImportMap`：导入映射表，用于将导入项和定义一一对应起来。

3. `Query`：导入项查询结果，包含了导入项的详细信息。

4. `InPrivateModule`：一个标记，表示对应的定义在私有模块中。

5. `Pub`：一个标记，表示对应的定义是公共的（public）。

6. `Priv`：一个标记，表示对应的定义是私有的（private）。

7. `Def`：表示一个定义（definition），可以是函数、类型、模块等。

8. `S`：一个标记，表示对应的定义是静态的（static）。

9. `Fmt`：一个标记，表示对应的定义支持格式化操作。

10. `NotImportableFromMain`：一个标记，表示对应的定义无法从主模块导入。

11. `fmt`：一个标记，表示对应的定义支持格式化输出。

12. `FMT`：一个标记，表示对应的定义是 `fmt` 宏。

### 特质（trait）：

1. `Display`：该特质用于定义实现了格式化输出的类型，使其可以通过 `format!` 宏进行字符串格式化操作。

### 枚举（enum）：

1. `SearchMode`：用于控制导入项的搜索模式。具体的模式包括 `Exact`（完全匹配）、`Suffix`（后缀匹配）和 `Substring`（子字符串匹配）。

2. `AssocSearchMode`：用于控制关联项的搜索模式。具体的模式包括 `Exact`、`Prefix`（前缀匹配）和 `Substring`。

这些结构体、特质和枚举的设计目的是为了在解析代码和语义分析过程中，提供相关的数据结构和方法，以便处理导入语句并建立导入项与定义之间的映射关系。同时，这些结构体、特质和枚举还提供了一些标记和选项，用于控制导入项和定义的可见性、搜索方式等。

