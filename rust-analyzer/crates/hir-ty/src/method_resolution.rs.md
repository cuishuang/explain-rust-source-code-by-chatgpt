# File: rust-analyzer/crates/hir-ty/src/method_resolution.rs

在rust-analyzer中，rust-analyzer/crates/hir-ty/src/method_resolution.rs文件的作用是处理方法解析的相关逻辑。具体来说，它负责确定给定类型上可用的方法、查找方法的实现、方法调用的解析、以及隐式方法解析等操作。

在该文件中，有三个结构体：TraitImpls、InherentImpls和ReceiverAdjustments。这些结构体分别用于存储特质实现、自定义类型上的方法实现和接收器调整。

- TraitImpls：用于存储特质上的方法实现，包括特质方法的签名和实现的位置。
- InherentImpls：用于存储自定义类型上的方法实现，包括类型的方法签名和实现的位置。
- ReceiverAdjustments：用于存储调整过的接收器类型，以便于解析方法调用中的self参数。

另外，还有几个枚举类型在这个文件中定义：

- TyFingerprint：用于表示类型的"指纹"，用于快速检查类型的相等性。
- LookupMode：用于指定方法查找的模式，包括查找特定实例方法、查找未绑定的方法和查找特质方法等。
- VisibleFromModule：用于表示方法是否可见于给定模块。
- IsValidCandidate：用于表示方法是否是有效的候选方法。

这些枚举类型在方法解析过程中提供了额外的信息和选项，以便按照特定的规则进行方法查找和解析。

