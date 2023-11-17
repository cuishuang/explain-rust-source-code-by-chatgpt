# File: vector/src/sinks/aws_s_s/retry.rs

在Rust生态的vector项目中，`vector/src/sinks/aws_s_s/retry.rs`文件的作用是实现了向AWS Simple Storage Service (S3)发送请求时进行重试的逻辑。当发送请求失败时，此文件中的代码将重试请求，直到达到最大重试次数或请求成功为止。

`SSRetryLogic<E>`是一个泛型结构体，用于定义S3请求的重试逻辑。它具有以下成员变量和方法：

- 成员变量：
  - `retries`: 表示已尝试的重试次数。
  - `max_retries`: 表示最大重试次数。
  - `error_codes`: 表示可以重试的错误码列表。
  - `error_statuses`: 表示可以重试的HTTP状态码列表。

- 方法：
  - `new(max_retries: u32) -> Self`: 创建一个新的`SSRetryLogic`实例，指定最大重试次数。
  - `can_retry(&self, error: &E) -> bool`: 判断给定的错误是否可以重试。
  - `next_retry(&mut self) -> bool`: 增加重试次数并检查是否还可以重试。

`SSRetryLogic<E>`结构体的对象用作创建AWS S3请求时的重试策略。根据给定的错误码和HTTP状态码，可以决定是否继续重试请求。

