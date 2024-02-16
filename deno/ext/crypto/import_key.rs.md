# File: /Users/fliter/rust-contribute/deno/ext/crypto/import_key.rs

在Deno项目的源代码中，/Users/fliter/rust-contribute/deno/ext/crypto/import_key.rs文件是负责导入密钥的功能。

该文件中定义了一些结构体和枚举，用于处理导入密钥的不同情况。

1. ECParametersSpki结构体：该结构体定义了使用ECParameters算法的公钥信息。其中包含了一些参数，如曲线类型、曲线坐标等，用于构建和解析ECDSA密钥。

2. KeyData枚举：该枚举定义了导入密钥时的数据类型。包括以下几种情况：
   - Raw：密钥数据是原始的二进制数据。
   - Spki：密钥数据是使用SubjectPublicKeyInfo编码的公钥。
   - Pkcs8：密钥数据是使用Pkcs8编码的私钥。
   - Jwk：密钥数据是使用JSON Web Key格式编码的密钥。

3. ImportKeyOptions枚举：该枚举定义了导入密钥时的选项参数，包括以下几种情况：
   - Format：指定密钥的数据格式，可以是KeyData枚举中的值。
   - Type：指定密钥的类型，可以是字符串类型的标识符，如"public"、"private"等。
   - Algorithm：指定密钥的算法，如"RSASSA-PKCS1-v1_5"、"ECDSA"等。

4. ImportKeyResult枚举：该枚举定义了密钥导入结果的返回类型。包括以下几种情况：
   - Success：密钥导入成功，并返回导入的密钥。
   - Error：密钥导入过程中出现错误，包括无效的密钥数据、格式、类型等。

