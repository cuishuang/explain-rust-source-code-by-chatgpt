# File: vector/src/enrichment_tables/geoip.rs

在Rust生态vector项目中，`vector/src/enrichment_tables/geoip.rs`文件是用于实现与地理位置相关的IP地址查询功能的模块。

该文件中定义了几个重要的结构体和枚举类型，包括`GeoipConfig`、`Geoip`和`DatabaseKind`。

`GeoipConfig`结构体用于配置地理位置查询模块，包含以下字段：
- `database_path: Option<PathBuf>`：指定GeoIP数据库文件的路径，如果未指定，则使用默认的数据库文件。
- `lookup_type: LookupType`：指定IP地址查询类型，包括`IP`（只查询IP地址）和`City`（查询IP地址及城市信息）。
- `skip_private_ip: bool`：是否跳过私有IP地址的查询。

`Geoip`结构体是地理位置查询功能的主要实现，包含以下方法：
- `load_database`：加载GeoIP数据库文件。
- `lookup_ip`：查询给定的IP地址，返回查询结果。

`DatabaseKind`枚举类型包含了支持的GeoIP数据库类型，包括：
- `GeoLite2Country`：GeoLite2国家数据库，提供国家级别的地理信息。
- `GeoLite2City`：GeoLite2城市数据库，提供更详细的城市级别的地理信息。

通过使用这些结构体和枚举类型，可以在Vector项目中开启地理位置查询功能，并根据需要设置相应的配置选项，从而对特定的IP地址进行查询并获取地理位置信息。

