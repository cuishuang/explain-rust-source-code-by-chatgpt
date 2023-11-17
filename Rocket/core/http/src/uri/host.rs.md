# File: Rocket/core/http/src/uri/host.rs

Rocket是一个基于Rust编写的轻量级、快速且安全的Web框架，它利用了Rust的静态类型和所有权系统来提供高性能、可维护性和安全性。在Rocket的源代码中，`Rocket/core/http/src/uri/host.rs`文件定义了与URI主机相关的功能。

在该文件中，定义了三个主要的结构体：`Host`, `Authority`和`Host<'a>(Authority<'a>)`。

1. `Host`: 它是存储URI主机的结构体，用于表示通过网络访问的URI的主机部分。`Host`结构体具有以下属性和方法：
   - `bytes(&self) -> &[u8]`: 获取URI主机的字节表示。
   - `split(&self) -> (&str, Option<u16>)`: 将URI主机分割为主机名和端口号，如果没有端口号则返回`None`。返回值为元组，第一个元素是主机名（字符串类型），第二个元素是可选的端口号（`Option<u16>`类型）。

2. `Authority`: 它是解析URI的主机和端口号的结构体。`Authority`结构体具有以下属性和方法：
   - `new(host: &'a str, port: Option<u16>) -> Result<Self, Error>`: 创建一个新的`Authority`实例，并验证解析后的主机和端口号是否有效。返回结果为`Result<Self, Error>`类型，其中`Self`表示`Authority`类型，`Error`表示解析错误的相关信息。
   - `host(&self) -> &str`: 获取解析后的主机名。
   - `port(&self) -> Option<u16>`: 获取解析后的端口号。

3. `Host<'a>(Authority<'a>)`: 它是一个包装了`Authority`结构体的泛型结构体，用于表示具体的URI主机。`Host`结构体主要提供了与解析URI主机相关的方法和属性，并委托给了内部的`Authority`结构体进行处理。

总结起来，`Rocket/core/http/src/uri/host.rs`文件中的三个结构体（`Host`, `Authority`和`Host<'a>(Authority<'a>)`）用于解析和表示URI的主机部分，并提供了访问和操作主机和端口号的方法。这些结构体在Rocket框架的核心HTTP功能中发挥着重要的作用，用于处理URI主机的相关操作。

