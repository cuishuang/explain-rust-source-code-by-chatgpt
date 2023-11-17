# File: vector/src/expiring_hash_map.rs

vector/src/expiring_hash_map.rs中的代码实现了一个带有过期时间的哈希映射，其中键值对在一定时间后会被自动删除。

以下是一些重要的结构体及其作用：

1. `ExpiringHashMap<K, V>`：这是一个哈希映射的主要结构体。它使用一个HashMap来存储键值对，并使用一个单独的HashMap来存储各个键的过期时间。此外，它还包含一个定时器线程，用于定期清理过期的键值对。

2. `Item<V>`：这是哈希映射中键值对的存储单元。它存储了具体的值V以及过期时间。

3. `ExpiringHashMapConfig`：这个结构体包含了一些配置参数，如清理线程的周期时间和存储桶的初始化大小。

主要功能和过程如下：

1. 在`ExpiringHashMap<K, V>`的实例化过程中，会创建一个存储键值对的HashMap和一个存储过期时间的HashMap。同时会启动一个定时器线程。

2. 当使用`ExpiringHashMap<K, V>`的`insert()`方法插入一个新的键值对时，会将键值对添加到HashMap中，并将当前时间加上键的过期时间存储到存储过期时间的HashMap中。如果键已经存在，则更新值和过期时间。

3. 定时器线程每隔一定的周期时间（由`ExpiringHashMapConfig`的配置确定），会检查存储过期时间的HashMap，将已过期的键从HashMap和存储过期时间的HashMap中移除，进而删除对应的键值对。这个过程确保了过期的键值对可以被自动删除。

4. 当使用`ExpiringHashMap<K, V>`的`get()`方法获取一个键对应的值时，会首先检查键是否过期。如果过期，则将其从HashMap和存储过期时间的HashMap中移除，并返回None。否则返回对应的值。

5. `ExpiringHashMap<K, V>`还提供了一些其他的功能，如`len()`方法用于获取哈希映射中键值对的个数，`contains_key()`方法用于检查键是否存在，`remove()`方法用于手动删除键值对等。

总结起来，`ExpiringHashMap<K, V>`实现了一个带有过期时间的哈希映射，使得键值对在一定时间后可以自动删除。这对于需要缓存数据或处理具有时效性的数据非常有用。

