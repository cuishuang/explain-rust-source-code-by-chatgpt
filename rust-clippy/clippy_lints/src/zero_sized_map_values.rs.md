# File: rust-clippy/clippy_lints/src/zero_sized_map_values.rs

在rust-clippy的源代码中，`zero_sized_map_values.rs`这个文件是用来实现一个 lint（一种代码检查工具）的。具体来说，它实现了一个 lint 规则，用于检查在使用 `HashMap` 或 `BTreeMap` 时，是否存在零大小（zero-sized）的值类型。

在 Rust 中，零大小类型是指占用零字节内存的类型。在一些情况下，使用零大小类型作为 HashMap 或 BTreeMap 的值类型可能会导致一些意外行为或错误使用。这个 lint 的目的就是帮助开发者在编译时发现这些潜在问题。

具体实现的细节可以在该文件中找到，下面是该文件中的部分代码注释的翻译：

```rust
// zero_sized_map_values.rs

// 避免使用零大小（zero-sized）的值类型作为 HashMap 或 BTreeMap 的值

use rustc_lint::{LateContext, LateLintPass, LintArray, LintPass};
use rustc_middle::lint::in_external_macro;
use rustc_ast::ast::PatKind;
use rustc_ast::visit::{FnKind, Visitor};
use rustc_hir as hir;

declare_clippy_lint! {
    /// **What it does:** This lint checks for usage of zero-sized map values
    ///
    /// **Why is this bad?** Spawning collections with zero-size values can lead to excessive memory
    /// allocation and slow performance.
    ///
    /// **Known problems:** None.
    ///
    /// **Example:** `HashMap<_, [u32; 0]>`
    pub ZERO_SIZED_MAP_VALUES,
    pedantic,
    "usage of zero-sized values in HashMap or BTreeMap"
}

pub struct ZeroSizedMapValues {
    in_trait_impl: bool,
    // ...
}

// ...
```

该文件中的 `ZeroSizedMapValues` 结构体实现了 `LateLintPass` trait，这个 trait 提供了检查 lint 的功能。具体的逻辑在 `check_expr` 和 `visit_pat` 等函数中实现。

总之，`zero_sized_map_values.rs` 是 `rust-clippy` 中用来检查在使用 `HashMap` 或 `BTreeMap` 时是否存在零大小值类型的规则的实现。它对于帮助开发者避免潜在的性能问题非常有用。

