# File: vector/lib/vector-stream/src/expiration_map.rs

vector-stream/src/expiration_map.rs是Vector项目中的一个文件，它的作用是实现了一个过期映射（expiration map）的数据结构。过期映射是一个键值对的存储结构，每个键都有一个过期时间，一旦时间超过设定的过期时间，该键值对就会被删除。

在这个文件中，有两个主要的结构体：`Emitter<T>`和`ExpirationMap<K, V>`。

`Emitter<T>`是一个泛型结构体，它主要用于将ExpiraitonMap中的某个键值对的值发送给一个回调函数。它包含以下字段：
- `value: Option<T>`：一个可选的泛型值，用于存储与键值对相关联的数据。
- `callback: Box<dyn Fn(T)>`：一个回调函数，当值被发送时，将会被调用。

`ExpirationMap<K, V>`是过期映射的主要结构体，它包含以下字段：
- `inner: Mutex<HashMap<K, (Option<Instant>, V)>>`：一个互斥锁（Mutex）的哈希映射（HashMap），它用于存储键值对和与其相关联的过期时间以及值。
- `remove_emitter: Option<Emitter<V>>`：一个可选的Emitter结构体，用于在删除过期键值对时，将其值发送给回调函数。
- `sleep_interval: Duration`：设置的休眠时间间隔，用于定期检查和删除过期键值对。

该结构体还实现了以下方法：
- `new()`：创建一个ExpiraitonMap结构体的实例。
- `insert(&self, key: K, value: V, expiration: Option<Duration>)`：插入一个键值对到过期映射中，并设置过期时间。
- `get(&self, key: &K) -> Option<V>`：从过期映射中获取一个键对应的值，如果键不存在或已过期，则返回None。
- `remove(&self, key: &K) -> Option<V>`：从过期映射中删除指定的键，并返回与之关联的值。
- `remove_expired(&self)`：删除所有已过期的键值对，并将它们的值发送给回调函数（如果已设置）。
- `start_cleanup_thread(&self)`：启动一个新的线程，定期检查和删除过期键值对。

总之，`ExpirationMap`结构体提供了一种方便的数据结构，可以在插入键值对时设置过期时间，并在指定时间内自动删除过期的键值对，并发送删除的值给回调。`Emitter`结构体则用于管理值的发送和回调函数的执行。

