# File: serde/serde_derive/src/internals/attr.rs

在serde/serde_derive/src/internals/attr.rs文件中，定义了一些用于处理attribute的数据结构和枚举类型。

首先，Attr<'c,'v>结构体表示一个属性的解析结果，包含两个泛型参数，'c表示上下文的类型，'v表示属性的值类型。
- `tags: Vec<TagType>`字段是一个标签类型的Vec，用于标识属性的名称或者别名。
- `body: Body`字段表示属性的主体部分，可以是一个字面值，也可以是一个标志属性。
- `span: Span`字段用于表示属性在源代码中的位置。

BoolAttr<'c>结构体表示一个布尔类型的属性。
- `value: bool`字段表示属性的布尔值。

VecAttr<'c,'attr>结构体表示一个列表类型的属性。
- `name: &'attr str`字段表示属性的名称。
- `values: Vec<Attr<'c,'attr>>`字段表示属性值的列表，每个属性值都是Attr类型。

Name枚举类型表示属性的名称，有两个变体：
- `Exact(Identifier)`表示属性名称与给定的标识符完全匹配。
- `FromPrefix(Identifier)`表示属性名称以给定的标识符开头。

RenameAllRules枚举类型表示属性的重命名规则，有三个变体：
- `None`表示不进行重命名。
- `LowerCase`表示将属性名称转为小写。
- `UpperCase`表示将属性名称转为大写。

Container枚举类型表示结构体的容器类型，有三个变体：
- `Struct`表示结构体。
- `TupleStruct`表示元组结构体。
- `Enum`表示枚举。

Variant枚举类型表示枚举变体，有两个变体：
- `Unit`表示无关联数据的变体。
- `Newtype(Identifier)`表示只有一个字段并带有名称的变体。

BorrowAttribute枚举类型表示属性的借用方式，有两个变体：
- `Borrowed`表示借用属性。
- `Owned`表示拥有属性。

Field枚举类型表示一个结构体字段的属性，有多个变体：
- `Regular`表示常规字段。
- `StructFlatten`表示一个结构体字段用于展开扁平化的属性。
- `StructFlattenRenameAll`表示一个结构体字段用于展开扁平化并进行重命名的属性。

TagType枚举类型表示属性标签的类型，有两个变体：
- `Exact(Identifier)`表示属性标签与给定的标识符完全匹配。
- `FromPrefix(Identifier)`表示属性标签以给定的标识符开头。

Identifier枚举类型表示标识符的类型，有两个变体：
- `Absolute`表示绝对路径标识符。
- `Relative(Pos)`表示相对路径标识符。

cannot枚举类型表示错误的类型，用于表示解析属性时可能出现的错误类型。

Default枚举类型表示属性的默认值，有两个变体：
- `Default`表示使用默认属性值。
- `Missing`表示属性值缺失。

