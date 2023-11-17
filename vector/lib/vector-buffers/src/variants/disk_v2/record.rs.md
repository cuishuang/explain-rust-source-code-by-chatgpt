# File: vector/lib/vector-buffers/src/variants/disk_v2/record.rs

文件record.rs是vector-buffers库中存储在磁盘上数据的记录相关的实现。它定义了Record结构体和RecordStatus枚举。

Record<'a>是一个泛型结构体，代表磁盘上的一条记录。它包含了一个引用类型的字段data，表示记录的数据，还有一个RecordStatus类型的字段status，表示记录的状态。

RecordStatus是一个枚举类型，定义了记录可能的几种状态。它包括：

- Written：表示记录已经被写入磁盘。
- Invalid：表示记录无效。
- Deleting：表示记录正在被删除。

这些状态反映了记录在存储和删除过程中可能的不同阶段。例如，当记录被写入磁盘后，它的状态将变为Written。如果记录被标记为无效，则状态为Invalid。在删除过程中，记录的状态为Deleting。

这些结构体和枚举定义了磁盘上记录的结构、状态和生命周期。在vector-buffers库中，它们被用于管理磁盘上的数据，包括记录的创建、写入和删除等操作。

