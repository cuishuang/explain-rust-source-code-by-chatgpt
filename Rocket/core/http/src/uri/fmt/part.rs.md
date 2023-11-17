# File: Rocket/core/http/src/uri/fmt/part.rs

在Rocket web框架的源代码中，Rocket/core/http/src/uri/fmt/part.rs这个文件的作用是定义URI的各个部分。

Part是一个trait，它定义了URI的不同部分，包括scheme、username、password、hostname、port、path、query和fragment。这个trait提供了对这些部分的访问方法，比如获取部分的字符串、获取部分的长度等。

Sealed也是一个trait，它限制了Part trait只能被当前 crate 实现。这样可以确保Part trait只能在Rocket框架中使用，不能被其他库实现或扩展。

Kind是一个enum，它定义了URI的种类，包括Absolute和Relative。Absolute表示绝对URI，包含完整的scheme和authority部分；Relative表示相对URI，不包含scheme和authority部分。

Path是一个enum，它定义了URI的path部分。它可以是Empty表示空路径，可以是Segment表示单独的路径段，也可以是MultiSegment表示多个路径段组成的路径。

Query是一个enum，它定义了URI的query部分。它可以是Empty表示空query，可以是Value表示一个键值对的query参数，也可以是MultiValue表示多个键值对的query参数。

这些enum类型的定义，为URI的各个部分提供了不同的类型表示，使得对URI的解析和构造更加方便和易懂。同时，由于这些类型是enum，可以利用模式匹配功能来处理不同的URI情况。

