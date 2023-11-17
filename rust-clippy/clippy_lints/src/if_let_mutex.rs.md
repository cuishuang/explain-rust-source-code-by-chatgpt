# File: rust-clippy/clippy_lints/src/if_let_mutex.rs

在rust-clippy库中，rust-clippy/clippy_lints/src/if_let_mutex.rs文件的作用是实现了一个lint（代码风格规范检查工具）用于检查使用`if let`表达式时是否应该使用`Mutex::lock`而不是`Mutex::try_lock`。这个lint主要用于检测并提醒开发者，在使用`if let`表达式时应该考虑到锁的竞争情况，以避免可能的线程安全问题。

OppVisitor，ArmVisitor这些struct是用于实现该lint的主要辅助结构体。

- `OppVisitor`是一个实现了`rustc_ast::visit::Visitor` trait的结构体。它用于遍历并访问语法树表达式中的`if let`块，检测其中使用了`Mutex::try_lock`而不是`Mutex::lock`的情况。该结构体的主要作用是递归地访问语法树，并通过`visit_expr`方法来触发对`if let`块的检查。
  
- `ArmVisitor`是一个实现了`rustc_ast::visit::Visitor` trait的结构体。它用于遍历并访问`if let`表达式的每个分支（即每个`arm`），检测是否存在使用`Mutex::lock`的条件。该结构体的主要作用是检测`if let`表达式的每个分支，通过`visit_expr`方法来触发对分支中锁的使用情况的检查。

通过这两个结构体，lint可以遍历语法树中的`if let`表达式，访问`if let`块以及其各个分支，并检查是否使用了正确的锁操作。

这样的设计很典型，通过遍历语法树并使用不同的结构体来处理不同的语法结构，可以较为方便地实现各种lint规则的检查。

