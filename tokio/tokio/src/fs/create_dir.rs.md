# File: tokio/tokio/src/fs/create_dir.rs

tokio源代码中的tokio/tokio/src/fs/create_dir.rs文件的作用是实现在异步上下文中创建目录的功能。

这个文件包含了一个名为create_dir的函数，它使用tokio的异步运行时来创建目录。该函数的定义如下：

```rust
pub fn create_dir<P>(path: P) -> CreateDirFuture<P>
where
    P: AsRef<Path>,
```

`create_dir`函数接受一个类型为`AsRef<Path>`的参数`path`，它表示要创建的目录的路径。`AsRef`是一个通用的路径转换trait，可以接受多种形式的路径，如字符串、Path对象等。

`CreateDirFuture`是一个返回类型为`Future`的结构体，它表示一个异步创建目录操作的未完成的结果。`CreateDirFuture`实现了`Future` trait，并且在`poll`方法中实现了具体的异步逻辑。

`create_dir`函数的实现主要分为两部分：先通过`tokio::fs::create_dir`函数创建设置好的目录，并将返回的`std::io::Result`转换为`Future`的结果类型；然后使用`tokio::spawn`函数将创建目录的`Future`异步运行并返回。

下面是部分`CreateDirFuture`的实现代码：

```rust
#[derive(Debug)]
pub struct CreateDirFuture<P>
where
    P: AsRef<Path>,
{
    state: CreateDirFutureState,
    dir: Option<PathBuf>,
    path: P,
}

#[derive(Debug)]
enum CreateDirFutureState {
    CreateDir(tokio::fs::CreateDirFuture),
    Complete(Option<std::io::Error>),
}

impl<P> Future for CreateDirFuture<P>
where
    P: AsRef<Path>,
{
    type Output = Result<(), std::io::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match self.state {
                CreateDirFutureState::CreateDir(ref mut create_dir) => {
                    match create_dir.poll_unpin(cx) {
                        Poll::Ready(result) => { ... },
                        Poll::Pending => return Poll::Pending,
                    }
                }
                ...
            }
        }
    }
}
```

在`CreateDirFuture`中，`state`字段用于记录目前的状态。`dir`字段表示创建的目录路径，`path`字段表示要创建目录的原始路径。

在`poll`方法中，使用一个循环来处理不同的状态。如果状态是`CreateDirFutureState::CreateDir`，则调用`poll_unpin`方法来异步创建目录。根据返回的结果，如果目录创建成功，则返回`Poll::Ready(Ok(()))`表示完成；如果目录创建失败，则返回`Poll::Ready(Err(err))`表示出错；如果创建目录操作仍在进行中，则返回`Poll::Pending`表示暂时无法完成。

此外，根据需要，`CreateDirFuture`还可以实现取消操作、超时设置等更加复杂的异步逻辑。

总之，tokio的`create_dir`函数通过tokio的异步运行时实现了在异步上下文中创建目录的功能，并提供了与其他tokio异步操作相同的能力和特性。

