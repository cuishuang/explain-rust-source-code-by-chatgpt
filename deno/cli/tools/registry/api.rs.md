# File: /Users/fliter/rust-contribute/deno/cli/tools/registry/api.rs

在Deno项目的源代码中，/Users/fliter/rust-contribute/deno/cli/tools/registry/api.rs文件是用于实现与Denoland包注册表API相关的功能。该文件定义了一系列结构体，包括CreateAuthorizationResponse、ExchangeAuthorizationResponse、User、OidcTokenResponse、PublishingTaskError、PublishingTask和ApiError。

- CreateAuthorizationResponse结构体表示创建授权响应的结果。它包含了返回的令牌和其他相关信息。

- ExchangeAuthorizationResponse结构体表示交换授权响应的结果。它包含了返回的令牌和其他相关信息。

- User结构体表示一个用户的信息，包括用户ID、电子邮件和用户名等。

- OidcTokenResponse结构体表示OpenID Connect（OIDC）令牌的响应结果。它包含了访问令牌、刷新令牌和令牌类型等信息。

- PublishingTaskError结构体表示发布任务的错误信息。它包含了错误码和错误描述等信息。

- PublishingTask结构体表示一个发布任务的信息，包括任务ID、提交者ID和任务状态等。

- ApiError结构体表示API操作过程中的错误信息。它包含了错误码和错误描述等信息。

