# File: rust-clippy/clippy_lints/src/methods/filter_map_next.rs

在Rust-Clippy源代码中，`filter_map_next.rs`文件的作用是实现`filter_map_next` lint。`filter_map_next`是一个自定义的lint规则，用于检测使用`filter_map`和`next`方法的代码，并建议将其替换为`filter_map_next`方法的组合。

该lint的目的是通过将`filter_map`与`next`方法结合使用来提高代码的可读性和性能。在某些情况下，我们可能需要使用`filter_map`方法来过滤和映射一个迭代器，然后再应用`next`方法来获取下一个元素。然而，这可以通过使用`filter_map_next`方法来更简洁地实现。

具体而言，`filter_map_next`方法是一个自定义的扩展方法，它在`Iterator` trait里添加了一个新的方法。该方法首先使用`filter_map`过滤和映射元素，然后返回第一个非`None`值。这样，我们可以在一次迭代中完成过滤、映射和获取下一个元素的操作。

`filter_map_next` lint会检查用户在代码中使用`iter.filter_map(|x| f(x).next())`的情况，并建议将其替换为`iter.filter_map_next(|x| f(x))`以提高代码的可读性。

总结而言，`filter_map_next.rs`文件实现了自定义的lint规则，用于检测并建议替换使用`filter_map`和`next`方法的代码，以提高代码的可读性和性能。

