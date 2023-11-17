# File: Rocket/core/http/src/header/content_type.rs

在Rocket web框架的源代码中，Rocket/core/http/src/header/content_type.rs文件负责解析和处理HTTP请求头中的Content-Type字段。Content-Type字段用于指示请求或响应的实体（payload）的媒体类型。

在该文件中，有以下几个struct：

1. ContentType
   - 这个struct是对Content-Type字段的包装，它提供了各种方法来解析和构建Content-Type字符串。
   - 方法：
     - `fn new(mime: Mime) -> Self`：通过提供一个Mime类型，创建一个新的ContentType实例。
     - `fn top_level(&self) -> &str`：获取Content-Type字段中的顶级媒体类型。
     - `fn sub_level(&self) -> &str`：获取Content-Type字段中的子级媒体类型。
     - `fn charsets(&self) -> Option<&Vec<Charset>>`：获取Content-Type字段中的字符集。
     - `fn is_top_level(&self, top_level: &str) -> bool`：检查Content-Type字段中的顶级媒体类型是否与给定的顶级媒体类型匹配。
     - `fn is_sub_level(&self, sub_level: &str) -> bool`：检查Content-Type字段中的子级媒体类型是否与给定的子级媒体类型匹配。

2. Charset
   - 这个struct表示Content-Type字段中的字符集。
   - 字段：
     - `pub mime: Mime`：字符集对应的Mime类型。
     - `pub charset: Charset`：具体的字符集。

3. ParsedContentType
   - 这个struct用于解析和表示Content-Type字段的各个组成部分。
   - 字段：
     - `pub top_level: String`：Content-Type字段中的顶级媒体类型。
     - `pub sub_level: String`：Content-Type字段中的子级媒体类型。
     - `pub params: Vec<(String, String)>`：Content-Type字段中的参数。

这些struct提供了对Content-Type字段的解析和构建的功能，使得开发者能够方便地处理HTTP请求和响应中的Content-Type信息。它们可以用于检查媒体类型、字符集等，以便进行适当的处理和响应。

