# File: rust-clippy/clippy_lints/src/disallowed_methods.rs

在rust-clippy的源代码中，disallowed_methods.rs文件的作用是定义了禁止使用的方法的lint规则。

首先，disallowed_methods.rs文件中定义了一个名为`DisallowedMethods`的struct，该struct用于存储禁止使用的方法的信息。每个`DisallowedMethods`实例都表示一个禁止使用的方法，包括方法的名称、对应的lint规则以及给出的建议。

在`DisallowedMethods` struct中，有以下几个字段：
- `method_names`: Vec<String>类型，存储禁止使用的方法的名称。
- `lint`: &'static str类型，表示对应的lint规则名称。
- `advice`: &'static str类型，给出使用替代方法的建议。

`DisallowedMethods` struct还提供了一些方法来创建和操作禁止使用的方法。例如，`DisallowedMethods::new`方法用于创建一个新的`DisallowedMethods`实例，可以指定禁止使用的方法的名称、对应的lint规则和建议。还有`DisallowedMethods::banned`方法用于获取预定义的常见禁止使用的方法列表。

除了`DisallowedMethods` struct，disallowed_methods.rs文件还提供了一个`register_disallowed_methods_lints`的函数，该函数用于注册lint规则，并在调用时使用了`DisallowedMethods` struct中定义的方法进行了禁止使用的方法的检查。

总而言之，disallowed_methods.rs文件的作用是定义了禁止使用的方法的lint规则，通过`DisallowedMethods` struct存储禁止使用的方法的信息，并提供了相关方法用于创建、操作禁止使用的方法，并通过`register_disallowed_methods_lints`函数注册lint规则并进行相应的检查。

