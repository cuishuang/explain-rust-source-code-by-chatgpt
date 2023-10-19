# File: tokio/tokio-util/src/either.rs

tokio-util/src/either.rs 是 tokio-util 库中的一个文件，它定义了一个枚举类型 Either<L, R>，以及与该枚举类型相关的一些实用方法和函数。

首先，让我们来了解一下 Either<L, R> 枚举的作用。在编程中常常会遇到需要在两个不同类型中选择一个的情况，而 Either<L, R> 为我们提供了一个方便的方式来表示这种选择。Either<L, R> 有两个泛型参数，L 和 R，分别代表左侧和右侧的类型。

在实际应用中，Either<L, R> 可以用来表示一种"要么是 A, 要么是 B"的情况。例如，可以用 Either<Error, Value> 来表示一个结果，它要么是一个错误，要么是一个值。这样的表示方式在处理异步操作或者结果返回时非常有用。

在 either.rs 文件中，Either<L, R> 枚举类型定义了如下的两个变体（variants）：

1. Left(L)：左侧的变体，表示 Either<L, R> 的值是 L 类型的值。
2. Right(R)：右侧的变体，表示 Either<L, R> 的值是 R 类型的值。

这样，我们就可以根据 Either<L, R> 枚举的值的具体类型来进行不同的操作和处理。比如，可以使用 match 语句来对 Either<L, R> 进行模式匹配，根据具体类型执行不同的代码逻辑。

tokio-util/src/either.rs 中还定义了许多与 Either<L, R> 相关的实用方法和函数，包括：
- `impl<L, R> Either<L, R>`：Either<L, R> 的实现块，包含了一些实用方法和函数。
- `impl<L, R> Either<L, R>` 其中 Right 类型的实现块：Right<R> 类型的特定实现，用于 Right 类型的特定操作和处理。
- `impl<L, R> Either<L, R>` 其中左侧类型的实现块：Left<L> 类型的特定实现，用于左侧类型的特定操作和处理。
- `impl<L, R, Target> Either<L, R>`：Either<L, R> 到 Target 类型的转换实现，可以将 Either<L, R> 转换为其他类型。
- `impl<A, B, L, R> Either<L, R>`：用于 Either<L, R> 与 (A, B) 类型之间的转换的实现。

总之，tokio-util/src/either.rs 文件定义了 Either<L, R> 枚举及其相关实用方法和函数，提供了一种方便的选择两个不同类型的方式。通过这个枚举类型，我们可以根据具体情况来选择执行不同的代码逻辑，并且能够方便地进行类型转换和处理。

