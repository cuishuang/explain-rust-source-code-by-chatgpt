# File: Rocket/core/http/src/header/media_type.rs

文件`media_type.rs`位于Rocket核心模块的`http`模块下，主要是处理HTTP请求和响应中的`Content-Type`和`Accept`头部字段的媒体类型。

在`media_type.rs`文件中，定义了`MediaType`结构体。`MediaType`表示一个媒体类型，包含三个字段：`type_`、`subtype`和`params`。`type_`和`subtype`字段表示媒体类型的主类型和子类型，例如`text`是主类型，`html`是子类型。`params`字段是一个可选的`MediaParams`枚举，表示媒体类型的参数，例如`charset=utf-8`。

`MediaParams`是一个枚举类型，表示媒体类型的参数。它包含四个变体：`Boundary`, `Charset`, `Filename`和`Other`。`Boundary`变体用于multipart类型的边界参数。`Charset`变体表示字符集参数。`Filename`变体表示文件名参数，该参数通常用于文件下载。`Other`变体表示除了边界、字符集和文件名之外的其他参数。

`Source`是另一个枚举类型，在Rocket中用于表示媒体类型的来源。它包含三个变体：`FromRequest`, `FromResponse`和`FromMethod`. `FromRequest`表示媒体类型来自HTTP请求，`FromResponse`表示媒体类型来自HTTP响应，`FromMethod`表示媒体类型来自HTTP请求方法。

`MediaType`结构体和相关枚举类型的定义提供了对媒体类型的解析和生成的功能，可以在Rocket框架中用于处理HTTP请求和响应中的媒体类型相关的操作。它们使得开发者能够方便地操作和判断媒体类型，以提供更加灵活和定制化的HTTP服务。

