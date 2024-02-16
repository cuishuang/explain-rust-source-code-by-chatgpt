# File: /Users/fliter/rust-contribute/deno/ext/ffi/turbocall.rs

在Deno项目的源代码中，/Users/fliter/rust-contribute/deno/ext/ffi/turbocall.rs这个文件的作用是实现Deno与Rust之间的函数调用。该文件定义了一系列结构体和枚举，用于将Deno函数的调用参数传递给Rust函数，并返回调用结果。

具体来说，该文件中的Trampoline结构体用于保存函数的指针和参数，以及相关的调用信息。通过调用Trampoline的run方法，可以执行真正的Rust函数，并将结果返回给Deno。

SysVAmd64、Aarch64Apple、Win64这几个结构体表示不同平台下的ABI（Application Binary Interface，应用二进制接口）参数传递规则。它们分别存储了各个平台下不同类型的参数传递方式。

Floating、Integral、Size、Param这几个枚举表示了不同类型的参数。Floating枚举包含了浮点数类型的参数，Integral枚举包含了整数类型的参数，Size枚举包含了指针或长度类型的参数，Param枚举包含了通用的参数类型。

通过使用这些结构体和枚举，Deno可以将函数调用参数按照平台规则传递给Rust函数，并获取执行结果。这样一来，在Deno中调用Rust函数的过程变得更加方便和高效。

