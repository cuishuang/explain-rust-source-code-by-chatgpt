# File: /Users/fliter/rust-contribute/deno/ext/crypto/export_key.rs

在Deno项目的源代码中，/Users/fliter/rust-contribute/deno/ext/crypto/export_key.rs文件的作用是实现了与密钥导出相关的功能。

该文件中定义了以下几个结构体：

1. ExportKeyOptions: 这个结构体用于指定密钥导出的选项。它包含了以下字段：
   - `format`: 导出密钥的格式，类型为`ExportKeyFormat`枚举。
   - `algorithm`: 导出密钥的算法，类型为`ExportKeyAlgorithm`枚举。
   - `extractable`: 导出的密钥是否可被外部使用。
   - `usages`: 导出的密钥的使用方式。

2. ExportKeyFormat: 这个枚举类型定义了密钥导出的格式，包括以下几种选项：
   - `Raw`: 导出为原始字节数组。
   - `Spki`: 导出为SubjectPublicKeyInfo格式。
   - `Pkcs8`: 导出为PKCS #8格式。

3. ExportKeyAlgorithm: 这个枚举类型定义了密钥导出的算法，包括以下几种选项：
   - `AesCtr`: 使用AES-CTR算法导出。
   - `AesCbc`: 使用AES-CBC算法导出。
   - `AesGcm`: 使用AES-GCM算法导出。
   - `RsaSsaPkcs1v1_5`: 使用RSA签名算法（PKCS #1 v1.5）导出。
   - `RsaPss`: 使用RSA签名算法（PSS）导出。
   - `Ecdsa`: 使用ECDSA算法导出。
   - `Ecdh`: 使用ECDH算法导出。

4. ExportKeyResult: 这个枚举类型表示密钥导出的结果，包括以下几种选项：
   - `Success`: 导出成功。
   - `InvalidKey`: 无效的密钥。
   - `OperationNotAllowed`: 不允许的操作。
   - `UnsupportedAlgorithm`: 不支持的算法。
   - `Other`: 其他错误。

这些结构体和枚举类型共同提供了密钥导出时的各种选项和结果，方便在相关的函数中进行参数的传递和返回结果的处理。

