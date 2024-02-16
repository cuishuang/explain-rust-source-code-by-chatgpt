# File: /Users/fliter/rust-contribute/deno/ext/net/ops.rs

在Deno项目的源代码中，/Users/fliter/rust-contribute/deno/ext/net/ops.rs这个文件的作用是处理网络操作相关的功能。这个文件中定义了一些结构体和枚举，用于支持网络操作的底层实现。

1. TlsHandshakeInfo结构体：用于保存TLS握手过程中的相关信息，包括证书、私钥等。

2. IpAddr结构体：表示IP地址的类型，可以是IPv4或者IPv6。

3. TcpListenerResource结构体：表示TCP监听器资源，用于在特定端口上监听传入的TCP连接请求。

4. UdpSocketResource结构体：表示UDP套接字资源，用于收发UDP数据报。

5. ResolveAddrArgs结构体：用于保存解析地址的参数，包括主机名和端口号。

6. ResolveDnsOption结构体：表示解析DNS配置的选项。

7. NameServer结构体：表示DNS服务器的信息，包括地址和端口号。

8. TestPermission结构体：表示测试权限的结果。

这些结构体主要用于在网络操作中传递和保存相关的信息，如地址、配置等。

而DnsReturnRecord枚举用于表示DNS解析返回的记录类型，包括A、AAAA、CNAME、TXT、SRV等不同的记录类型。这些枚举值表示了不同类型的DNS记录，可以用于解析和处理DNS查询的结果。

