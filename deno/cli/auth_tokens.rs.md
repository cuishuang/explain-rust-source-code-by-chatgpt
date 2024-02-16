# File: /Users/fliter/rust-contribute/deno/cli/auth_tokens.rs

在Deno项目的源代码中，`auth_tokens.rs`文件定义了与身份验证令牌有关的结构和逻辑。

首先，`AuthToken`结构表示一个身份验证令牌，它包含以下字段：
- `file_name`: 所属的文件名。
- `token`: 身份验证令牌对应的字符串。

其次，`AuthTokens`结构表示一组身份验证令牌，它内部维护了一个 `Vec<AuthToken>`，用于存储多个身份验证令牌。主要提供了以下功能：
- `new()`: 创建一个空的 `AuthTokens` 实例。
- `get_auth_token()`: 根据文件名获取对应的身份验证令牌。
- `add_auth_token()`: 向 `AuthTokens` 实例中添加一个身份验证令牌。
- `remove_auth_token()`: 根据文件名移除对应的身份验证令牌。

最后，`AuthTokenData`枚举是一个用于描述身份验证令牌的数据结构。它分为以下几种类型：
- `Fs`: 表示从文件系统加载的身份验证令牌数据。
- `Http`: 表示从网络请求中获取的身份验证令牌数据。
- `InMemory`: 表示内存中保存的身份验证令牌数据。
- `Missing`: 表示找不到所需的身份验证令牌数据。

`AuthTokenData`主要作为一个封装，用于统一处理从不同来源获取的身份验证令牌数据。

