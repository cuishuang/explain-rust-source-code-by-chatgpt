# File: tokio/tokio/src/io/util/buf_reader.rs

在Tokio源代码中，`tokio/src/io/util/buf_reader.rs`这个文件实现了一个包装器(`Wrapper`)类型 `BufReader<R>`，该类型封装了一个`Read` trait的实现对象(R)，提供了带有缓冲区的读取功能，用于提高读取数据时的效率。

`BufReader<R>`是一个具有缓冲区的读取器。它实现了`AsyncRead` trait，并且可以通过调用`read`方法异步读取数据。与直接从底层读取器读取数据相比，使用BufReader可以减少系统调用的次数，从而提高读取速度。

BufReader的内部维护了一个缓冲区，它的大小由构造BufReader对象时指定的`buf_size`参数决定。读取数据时，BufReader首先尝试从缓冲区中读取数据，如果缓冲区为空，则从底层读取器(R)中填充缓冲区，并返回部分或全部缓冲区的内容。

BufReader有以下几个主要的结构体:

1. `BufReader<R>`: BufReader的主结构体，实现了`AsyncRead` trait，提供了读取功能。

2. `Parts`: BufReader的持有者结构体，包含了一个底层的读取器(R)。

3. `IntoRead`：包含了一个读取器(R)，通过实现`From<R>` trait用于将读取器转换为BufReader。

4. `SeekState`：表示BufReader的`seek`操作的状态，是一个enum类型，在BufReader中用于处理在读取数据时进行seek操作的情况。

SeekState的主要枚举值有以下几个：

1. `Waiting`: 表示BufReader正在等待`read_buf`方法的返回结果，在执行seek操作时需要刷新缓冲区。
2. `Buf`: 表示BufReader的缓冲区包含了seek地址之前的部分数据，并准备好继续读取数据。
3. `Seek`: 表示BufReader处于seek操作的状态，需要清空缓冲区并重新填充新的数据。
4. `Empty`: 表示BufReader的缓冲区为空，需要从底层读取器中填充数据。

这些结构体和枚举类型的组合使用，可以使得BufReader具备缓冲读取和seek操作的功能，提供了对底层读取器(R)的一种封装，更高效地读取数据。

