# File: Rocket/core/http/src/uri/authority.rs

在Rocket web框架的源代码中，文件Rocket/core/http/src/uri/authority.rs的作用是处理URI中的authority部分。URI是一个标识统一资源的字符串，包括协议、主机、端口号等信息。Authority模块负责解析和处理URI中的authority部分，即主机和端口号。

在该文件中，定义了一个名为Authority<'a>的结构体，用于表示URI中的authority部分。这个结构体具有以下几个重要的字段和方法：

1. 字段：
   - host：表示URI中的主机名，类型为Cow<'a, str>。该字段是一个借用字符串类型，可以是一个静态字符串，也可以是一个借用字符串。
   - port：表示URI中的端口号，类型为Option<u16>。该字段是一个可选的无符号16位整数，可以为Some(port)或None。
   - invalid_host_chars：表示在URI中不允许的主机名字符集合，类型为Vec<char>。默认情况下，包含了一些非法的主机名字符，可以通过方法add_invalid_char(char)来添加不允许的字符。

2. 方法：
   - new(host: Cow<'a, str>, port: Option<u16>)：构造方法，用于创建一个Authority实例。传入host和port参数，分别表示主机名和端口号。
   - host_and_port(&self) -> Cow<'a, str>：返回URI中的完整主机名和端口号字符串，形式为"host:port"。如果port为None，则只返回host，否则返回"host:port"。
   - add_invalid_char(&mut self, invalid_char: char)：添加一个非法的主机名字符到invalid_host_chars集合中。
   - is_valid_host_char(&self, ch: char) -> bool：判断一个字符是否为合法的主机名字符。如果该字符在invalid_host_chars集合中，则返回false，否则返回true。

这些方法和字段使得Authority结构体可以解析和处理URI中的authority部分，提取出主机名和端口号，并进行校验。它可以用于解析和构建URI，验证主机名和端口号的合法性，以及生成完整的主机名和端口号字符串。

