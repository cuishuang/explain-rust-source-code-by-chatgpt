# File: tokio/tokio/src/future/poll_fn.rs

在tokio源代码中，tokio/tokio/src/future/poll_fn.rs文件的作用是定义了一个用于创建自定义轮询器的宏和相应的结构体。

该文件中包含了两个结构体：`PollFn`和`PollFnMut`。这两个结构体都是泛型结构体，参数化了一个闭包类型，该闭包类型的签名必须符合指定的要求。

`PollFn`结构体的定义如下：

```rust
pub struct PollFn<F> where F: FnMut(&mut Context<'_>) -> Poll<T> { /* fields omitted */ }
```

该结构体实现了`Future` trait，用于表示一个将通过闭包进行轮询操作的未来。构造函数`poll_fn`接受一个满足特定要求的闭包作为参数，然后返回一个`PollFn`实例，该实例可以通过调用`poll`方法进行轮询。

`PollFnMut`结构体的定义如下：

```rust
pub struct PollFnMut<F> where F: FnMut(&mut Context<'_>) -> Poll<T> { /* fields omitted */ }
```

该结构体也实现了`Future` trait，与`PollFn`类似，不同之处在于闭包是可变的，因此可以在闭包中进行修改。

这两个结构体的主要作用是提供一种简便的方式来创建自定义的轮询器。闭包作为参数，定义了轮询器函数的行为，他们通过调用闭包的`FnMut`方法来进行轮询。这种方式将轮询逻辑与其他的未来逻辑分离，并将其封装在一个结构体中，提供了更高级别的抽象。

总而言之，`poll_fn.rs`文件中的`PollFn`和`PollFnMut`结构体提供了一种创建自定义轮询器的方法，通过传递一个满足特定要求的闭包来实现。这样的设计使得tokio能够更加灵活地处理不同类型的轮询逻辑。

