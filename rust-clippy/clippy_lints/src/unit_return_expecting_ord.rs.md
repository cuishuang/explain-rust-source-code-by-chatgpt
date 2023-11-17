# File: rust-clippy/clippy_lints/src/unit_return_expecting_ord.rs

在rust-clippy/clippy_lints/src/unit_return_expecting_ord.rs文件中，定义了一个名为`UNIT_RETURN_EXPECTING_ORD`的lint。这个lint用于检查函数返回类型为`()`，而在返回语句中使用了`cmp`、`max`、`min`等需要实现`Ord` trait的方法。

该lint的目的是防止在使用比较或求最值的方法时，错误地将返回类型设为`()`。因为比较或最值的方法需要返回实现`Ord` trait的类型，而`()`不是一个可比较的类型。因此，如果函数返回类型为`()`，而在返回语句中使用了这些方法，很可能是代码错误或逻辑错误。

该lint的实现方式是通过AST遍历源代码，检查每个函数的返回类型和返回语句。如果发现返回类型为`()`，而在返回语句中使用了需要实现`Ord` trait的方法，就会触发lint警告。

通过使用这个lint，可以帮助开发者在编译时捕获潜在的错误，提高代码质量和可靠性。

