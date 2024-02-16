# File: /Users/fliter/rust-contribute/deno/runtime/ops/web_worker/sync_fetch.rs

在Deno项目的源代码中，/Users/fliter/rust-contribute/deno/runtime/ops/web_worker/sync_fetch.rs文件的作用是处理主线程和Web Worker之间的同步脚本请求。

具体来说，SyncFetchScript文件中定义了名为`SyncFetchScript`的几个结构体，用于处理Web Worker中对脚本的同步获取请求。

1. `FetchOnceOptions`: 这个结构体定义了一些选项，如是否获取子资源、是否忽略缓存等。它作为`SyncFetchScript`的参数之一，用于传递请求的选项。

2. `FetchResponse`: 这个结构体包含了从网络上获取到的脚本的相关信息，例如脚本的内容、请求的URL等。它作为`SyncFetchScript`的返回类型，用于传递响应结果。

3. `FetchedRecord`: 这个结构体记录了已经被获取过的脚本的URL和内容等信息，以避免重复获取。它被用作`SyncFetchScript`的内部状态。

4. `SyncFetchScript`: 这个结构体实现了对脚本的同步获取逻辑。它包含了主要的处理方法，如`fetch_script_once`，用于从给定的URL获取脚本，并返回对应的`FetchResponse`。它还维护了一个`FetchedRecord`，用于记录已经获取过的脚本，以避免重复获取。

总的来说，SyncFetchScript结构体及其关联的其他结构体主要负责了Web Worker的同步脚本获取功能，提供了一个方便从网络获取脚本的接口，并且避免了重复获取。

