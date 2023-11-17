# File: Rocket/core/lib/src/fairing/ad_hoc.rs

在Rocket web框架的源代码中，`Rocket/core/lib/src/fairing/ad_hoc.rs` 文件是实现了一种特殊类型的中间件称为"AdHoc"的模块。AdHoc中间件允许用户自定义的函数作为中间件使用，并且可以在应用启动时执行一次性任务。

`AdHoc` 结构体定义了一种特殊类型的fairing，它包装了用户自定义的函数作为中间件使用。这个结构体有一个泛型参数 `F`，指定用户提供的函数的类型。这个函数的类型是 `Fn(&Rocket) -> rocket::fairing::Result<()>`，表示它接受一个 `Rocket` 实例的引用作为参数并返回一个 `Result` 类型。

`Once<F>` 结构体是 `AdHoc` 结构体的一个具体实现，用于在应用启动时执行一次性任务。它接受一个用户自定义的函数作为参数，并在 `attach()` 方法被调用时执行该函数。

`Normalizer` 结构体定义了一种中间件，用于对请求和响应进行一些标准化的操作。例如，它可以处理请求头部、响应头部、URI 等。

`AdHocKind` 枚举定义了不同类型的 `AdHoc` 组件，它有以下几个值：
- `OnResponse`：用于在每个响应发送之前执行一个函数。
- `OnRequest`：用于在每个请求开始之前执行一个函数。
- `PreRouter`：用于在路由匹配之前执行一个函数。
- `PreMatch`：用于在路由匹配之后，但在执行路由处理函数之前执行一个函数。
- `PostMatch`：用于在路由处理函数之后执行一个函数。

总之，`Rocket/core/lib/src/fairing/ad_hoc.rs` 文件中的代码实现了一组 AdHoc 中间件，允许用户在 Rocket 应用程序中灵活地定制和扩展中间件的功能。

