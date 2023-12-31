# File: tokio/tokio/src/util/rand.rs

文件`tokio/tokio/src/util/rand.rs`主要定义了用于生成随机数的工具类和结构体。

1. `RngSeed`结构体表示一个随机数生成器（RNG）的种子。它可以通过多种方式创建，包括从字节数组、时间戳和环境变量等生成。`RngSeed`提供了一些方法用于获取种子的信息以及序列化和反序列化种子。

2. `FastRand`结构体实现了一个简单的伪随机数生成器（PRNG）。它使用线性同余算法生成伪随机数序列。`FastRand`提供了一些方法用于生成不同范围的随机数，如生成无符号整数、浮点数和带范围的随机数。

这些工具类和结构体是为了简化和统一随机数的生成过程，以及提供一定程度的安全性和方便性。

