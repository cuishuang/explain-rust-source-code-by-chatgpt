# File: tokio/tokio/src/runtime/scheduler/inject.rs

在tokio源代码中，tokio/tokio/src/runtime/scheduler/inject.rs文件的作用是提供注入功能以控制并发任务的执行。

具体而言，Inject模块定义了几个struct，分别是：

1. Inject<T>：它是一个泛型类型，定义了一个注入功能的结构体。该结构体可以注入具有类型T的计算任务。Inject结构体包含一个Mutex，用于在执行任务时对任务存储进行同步访问。
   
   Inject结构体实现了Drop trait，这意味着当Inject结构体的实例被销毁时，会将其中存储的未执行的任务（Send包装的Box<dyn Future<Output = T>>）进行取消。

   Inject结构体还实现了几个方法：
   - new：创建一个新的注入功能实例。
   - inject：将一个任务注入注入功能实例中。
   - poll：遍历所有存储的未执行的任务，并尝试执行它们。

2. TaskInjector<T>：它是Inject<T>的一个只读引用结构体。主要用于获取Inject实例，并将任务注入其中。

   TaskInjector结构体实现了From<T> trait，这意味着可以从Inject实例中提取TaskInjector实例。

3. FutureInjector<T>：它是Inject<T>的一个持有所有权的结构体。与TaskInjector类似，它也用于获取Inject实例并将任务注入其中。不过与TaskInjector不同的是，FutureInjector在创建实例时会从Inject实例中获取所有权。

这些结构体共同使用，为tokio提供了一个灵活的调度器，并允许在运行时注入和取消任务。通过注入功能，用户可以动态地向tokio的任务队列中添加和删除任务，以实现更高级的调度和任务管理。

