# File: vector/src/sinks/s3_common/config.rs

在Rust生态的vector项目中，vector/src/sinks/s3_common/config.rs文件的作用是定义与S3（Amazon Simple Storage Service）相关的配置项和逻辑。

S3Options结构体用于表示与S3连接相关的选项，其中包含以下字段：
- `bucket`：S3存储桶的名称。
- `key_prefix`：存储桶中对象的前缀，用于对日志进行分组或分类。
- `region`：S3存储桶所在的AWS区域。
- `access_key`和`secret_access_key`：用于身份验证的访问密钥。

S3RetryLogic结构体定义了S3重试逻辑的各个方面。它包含以下字段：
- `max_attempts`：最大重试次数。
- `base`和`max`：指数退避算法的基础等待时间和最大等待时间。
- `jitter`：指数退避算法的抖动因子，用于在计算等待时间时引入随机性。

S3StorageClass、S3ServerSideEncryption、S3CannedAcl和HealthcheckError这几个枚举分别用于定义S3存储类、服务器端加密、预定义的访问控制策略和健康检查错误类型。

- S3StorageClass枚举定义了不同的存储类选项，如标准存储、低频访问存储等。通过该枚举，可以指定将日志存储为何种存储类型。
- S3ServerSideEncryption枚举用于选择在S3上存储日志时进行服务器端加密的方式，如AES-256。
- S3CannedAcl枚举定义了一些预定义的访问控制策略，用于控制日志对象的访问权限。
- HealthcheckError枚举包含与健康检查相关的错误类型，如无法连接到S3服务器或意外的HTTP响应等。

这些结构体和枚举的定义使得在使用vector针对S3进行配置时，可以方便地设置和管理相关的选项、逻辑和错误处理。

