# File: vector/src/sources/host_metrics/filesystem.rs

文件 vector/src/sources/host_metrics/filesystem.rs 是 Rust 生态 vector 项目中用于处理主机文件系统指标的源代码文件。

在这个文件中，定义了一个名为 Filesystem 的源，用于采集主机的文件系统指标信息。该源会定期扫描主机的文件系统，收集以下指标信息：

1. 文件系统的总容量（total）
2. 已使用的容量（used）
3. 剩余的容量（free）
4. 可以使用的容量（available）
5. 文件系统的使用率（usage）

这些指标信息可用于监控和分析主机的存储资源使用情况。

在文件系统源的实现中，定义了一个 FilesystemConfig 结构，用于存储配置信息。该结构中包含了以下字段：

1. `ignore_backup_partition`: 布尔类型，表示是否忽略备份分区的指标，默认为 true。
2. `ignore_nfs_volumes`: 布尔类型，表示是否忽略 NFS 卷的指标，默认为 true。
3. `ignore_tmpfs_volumes`: 布尔类型，表示是否忽略 tmpfs 卷的指标，默认为 true。
4. `scan_delay_secs`: 整数类型，表示扫描文件系统的延迟时间（以秒为单位），默认为 30。
5. `whitelist`: 字符串 Vec 类型，表示要扫描的文件系统列表，默认为空。

这些配置选项可以根据具体需求进行修改和设置，以适应不同的场景。

总结起来，文件 vector/src/sources/host_metrics/filesystem.rs 实现了一个主机文件系统的源，用于采集和处理主机文件系统的指标信息。通过配置 FilesystemConfig 结构，可以对采集的文件系统进行过滤和调整。这样，用户可以获取主机文件系统的使用情况，并进行监控和分析。

