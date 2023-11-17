# File: Rocket/examples/todo/src/task.rs

在Rocket框架的例子项目中的`Rocket/examples/todo/src/task.rs`文件中，定义了`Task`和`Todo`这两个结构体，并实现了与它们相关的功能。

首先，`Task`结构体表示一个具体的任务，包含了任务的id、标题、描述和是否已完成的状态。它的定义如下：

```rust
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
}
```

`Todo`结构体则表示一个任务列表，内部包含了多个`Task`对象，并提供相关操作方法。它的定义如下：

```rust
pub struct Todo {
    tasks: Vec<Task>,
}

impl Todo {
    /// 创建一个新的空任务列表
    pub fn new() -> Self {
        Todo { tasks: Vec::new() }
    }

    /// 返回任务列表中的所有任务
    pub fn all(&self) -> &[Task] {
        &self.tasks
    }

    /// 添加一个新的任务到任务列表中
    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    // ... 其他操作方法 ...
}
```

`Task`结构体表示一个具体的待办事项的属性，通过`title`和`description`分别表示待办事项的标题和描述，通过`completed`表示该待办事项是否已完成。

`Todo`结构体表示一个任务列表，通过`Vec<Task>`存储多个待办事项。它提供了添加任务、获取任务列表等基本操作，方便对任务列表进行管理。

在`task.rs`文件中，定义了`Task`和`Todo`结构体后，可以根据实际需求进行扩展和修改，并与Rocket框架的其他模块一起使用，完成一个简单的任务管理功能的web应用。

