# File: rust-clippy/clippy_lints/src/disallowed_names.rs

文件disallowed_names.rs的作用是存储和管理被禁止使用的标识符和关键字名称的列表。

DisallowedNames这个结构体定义了一个数组，其中包含了被禁止使用的名字列表。这个结构体实现了一些相关的方法，用于添加、移除和查询禁止使用的名称。

具体来说，DisallowedNames结构体包含以下几个重要的字段和方法：

1. `names: &'static [&'static str]`：这个字段是一个静态的字符串数组，用来存储被禁止使用的名称。

2. `new(names: &'static [&'static str]) -> Self`：这个方法用于创建一个DisallowedNames结构体的新实例，传入一个静态的字符串数组作为被禁止使用的名称列表。

3. `contains(&self, s: &str) -> bool`：此方法用于检查给定的名称是否在被禁止使用的名称列表中，并返回一个布尔值，表示是否存在。

4. `suggest_new_name(&self, s: &str) -> Option<String>`：此方法尝试为给定的名称生成一个新的建议名称，如果生成的建议名称不与已禁止使用的名称冲突，则返回Some建议名称，否则返回None。

DisallowedNames结构体的目的是为了辅助代码静态分析工具，如rust-clippy，在编译过程中检查代码中使用的标识符和关键字的合法性。通过这个结构体，可以轻松地定义和管理被禁止使用的名称列表，从而提高代码质量和可维护性。

