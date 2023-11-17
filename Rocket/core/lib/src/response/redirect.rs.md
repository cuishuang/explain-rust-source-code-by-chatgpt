# File: Rocket/core/lib/src/response/redirect.rs

在Rocket web框架的源代码中，`redirect.rs` 文件位于 `Rocket/core/lib/src/response` 目录下，其作用是实现了一个用于重定向的响应类型。

该文件定义了三个关键的结构体：`Redirect`、`PermanentRedirect` 和 `TemporaryRedirect`。

- `Redirect` 结构体表示一个重定向响应。它包含一个 `Status` 和一个 `uri` 字段。`Status` 是一个枚举类型，表示 HTTP 状态码，例如 `Ok`、`BadRequest` 等。`uri` 是一个 `Uri` 类型的变量，表示要重定向到的目标 URI。

- `PermanentRedirect` 结构体表示一个永久重定向响应。它是 `Redirect` 结构体的别名，使用了 `Status::PermanentRedirect` 枚举值表示 HTTP 301 状态码。

- `TemporaryRedirect` 结构体表示一个临时重定向响应。它是 `Redirect` 结构体的别名，使用了 `Status::TemporaryRedirect` 枚举值表示 HTTP 302 状态码。

这些结构体提供了方便的方式来创建重定向响应。通过创建一个 `Redirect`、`PermanentRedirect` 或 `TemporaryRedirect` 实例，可以指定目标 URI 和相应的状态码，然后可以将该实例返回给客户端，以完成重定向操作。

这个文件的作用是为 Rocket 提供了一个统一的方式来处理重定向，使开发者可以方便地创建和发送重定向响应。

