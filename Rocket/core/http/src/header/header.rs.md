# File: Rocket/core/http/src/header/header.rs

在Rocket web框架的源代码中，Rocket/core/http/src/header/header.rs文件的作用是定义了HTTP头部（headers）的相关结构体和方法。

该文件定义了两个主要的结构体：Header和HeaderMap。

1. Header结构体：该结构体用于表示一个HTTP头部的键值对。它具有以下属性和方法：
   - `pub name: Cow<'h, str>`：表示HTTP头部字段的名称。
   - `pub value: Cow<'h, str>`：表示HTTP头部字段的值。
   - `pub normalised: bool`：表示该头部是否已经归一化（即规范化）。
   - `pub parse<T: FromParam>(&self) -> Result<T, &'static str>`：根据头部值的类型尝试将其解析为指定的类型，并返回解析结果。
   
2. HeaderMap结构体：该结构体是一个动态的、存储了HTTP头部的映射表。它具有以下属性和方法：
   - `headers: Vec<Box<dyn Header<'h> + 'h>>`：表示HTTP头部的列表，每个头部都实现了Header trait。
   - `pub new() -> HeaderMap<'h>`：创建一个新的空的HeaderMap实例。
   - `pub get<H: Header<'h>>(&'h self) -> Option<&'h H>`：根据给定的HTTP头部类型获取对应的头部实例（如果存在）。
   - `pub get_raw(&'h self, name: &str) -> Option<&'h str>`：根据给定的头部字段名称获取对应的原始头部值（如果存在）。
   - `pub insert<H: Header<'h> + 'h>(&mut self, header: H) -> Result<(), String>`：插入一个新的HTTP头部到HeaderMap中，如果头部已经存在，则会更新对应的值。
   - `pub remove<H: Header<'h> + 'h>(&mut self)`：删除给定类型的HTTP头部。
   - `pub iter(&self) -> Iter<'_, Box<dyn Header<'h> + 'h>>`：返回HeaderMap的迭代器，可用于遍历所有的HTTP头部实例。
   
总结：
Header结构体表示一个HTTP头部的键值对，而HeaderMap结构体则是一个动态的、存储了HTTP头部的映射表。HeaderMap可以用于获取、插入、删除和遍历HTTP头部，并提供了一些方便的解析方法。这些结构体和方法组成了Rocket中对HTTP头部的处理功能，用于解析和操作HTTP请求和响应的头部信息。

