# File: Rocket/examples/cookies/src/main.rs

Rocket/examples/cookies/src/main.rs这个文件是Rocket框架中的一个示例应用程序文件，用于演示如何使用cookies（HTTP cookie）功能。

在该文件中，首先通过引入需要的依赖项来设置应用程序。其中包括：
- `rocket::http::{Cookie, Cookies}`：引入了Rocket框架中用于处理HTTP cookie的结构和函数。
- `rocket::response::content::Html`：引入用于生成HTML内容的结构和函数。

之后，定义了一个GET路由处理函数`index`，该函数的作用是在用户访问首页时发送一个包含cookie的HTML页面作为响应。这个函数使用了函数签名`fn index(cookies: Cookies) -> Html<&str>`，其中`Cookies`参数是用于处理cookie的结构，`Html<&str>`是表示响应的HTML内容。

在`index`函数中，首先通过`cookies.add()`函数创建了一个名为`message`的cookie，并将其值设置为`"Hello, cookies!"`。然后使用`Html`结构创建一个HTML内容，并将cookie的值作为其中的一个段落进行显示。最后，返回该HTML内容作为响应。

接下来，定义了一个GET路由处理函数`read`，该函数的作用是在用户访问/read路径时读取并显示已设置的cookie值。与`index`函数类似，`read`函数也使用了函数签名`fn read(cookies: Cookies) -> String`，其中`Cookies`参数是用于处理cookie的结构，`String`是响应的字符串内容。

在`read`函数中，通过`cookies`参数的`get`方法获取了名为`message`的cookie的值（如果存在）。然后，将其作为字符串返回。

最后，`main`函数中调用了`rocket::ignite().mount()`来启动Rocket应用程序，并将上述的GET路由绑定到相应的URL路径。

