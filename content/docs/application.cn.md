---
title: 应用
menu: docs_basics
weight: 140
---

# 编写应用

actix-web提供了用Rust构建Web服务器和应用程序的各种基础类型。它提供了路由，中间件，请求的预处理，响应的后处理，websocket协议处理，multipart流，等等。

所有actix web服务器都是围绕该App实例构建的。它用于为资源和中间件注册路由。它还存储同一应用程序中所有处理程序之间共享的应用程序状态

应用程序充当所有路由的命名空间，即特定应用程序的所有路由具有相同的url路径前缀。应用程序前缀总是包含一个前导的“/”斜杠。如果提供的前缀不包含前导斜杠，则会自动插入。前缀应该由路径值组成。

对于具有前缀的应用程序/app，与任何请求路径中有/app，/app/或/app/test匹配; 然而，路径/application不匹配。

{{< include-example example="application" section="make_app" >}}

在此示例中，将创建具有/app前缀和index.html资源的应用。该资源可通过/app/index.html路径获得。

>有关更多信息，请查看[URL Dispatch](../url-dispatch)部分。


一台服务器服务多个应用：

{{< include-example example="application" section="run_server" >}}

所有/app1请求路由到第一个应用程序，/app2到第二个，所有其他到第三个。 应用程序根据注册顺序进行匹配。如果具有更通用的前缀的应用程序在不通用的应用程序之前注册，它将有效地阻止较不通用的应用程序匹配。例如，如果App将前缀"/"注册为第一个应用程序，它将匹配所有传入的请求。

## 状态

同一应用程序内应用程序状态被所有路由和资源共享。当使用http actor时，状态可以HttpRequest::state()作为只读访问，但内部可变性RefCell可用于实现状态可变性。状态也可用于路由匹配谓词和中间件。

我们来编写一个使用共享状态的简单应用程序。我们打算将请求计数存储在状态中：

{{< include-example example="application" file="state.rs" section="setup" >}}

应用程序需要通过初始化状态来初始化。

{{< include-example example="application" file="state.rs" section="make_app" >}}

> **注意**：http服务器接受应用程序工厂而不是应用程序实例。Http服务器为每个线程构造一个应用程序实例，因此应用程序状态必须多次构建。如果你想在不同线程之间共享状态，应该使用共享对象，例如Arc。应用程序状态并不需要是Send和Sync，但应用程序的工厂必须是Send+ Sync。

要启动以前的应用程序，请为其创建闭包：

{{< include-example example="application" file="state.rs" section="start_app" >}}

## 结合不同状态的应用程序 

将多个应用程序与不同状态组合也是可能的。

[server::new](https://docs.rs/actix-web/*/actix_web/server/fn.new.html)需要handler具有单一类型。

使用[App::boxed](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.boxed)方法可以轻松解决此限制，该方法可将App转换为boxed trait object。

{{< include-example example="application" file="state.rs" section="combine" >}}

## 使用应用程序前缀来组合应用程序

该App::prefix()方法允许设置特定的应用程序前缀。此前缀表示将添加到所有资源模式的资源前缀通过资源配置。 这可以用来帮助挂载一组路由在不同的
地点比所包含的可调用作者的意图仍保持原样资源名称。

例如：

{{< include-example example="url-dispatch" file="prefix.rs" section="prefix" >}}

在上面的示例中，`show_users`路由将具有/users/show的有效路由模式， 而不是/ show，因为应用程序的前缀参数将预先添加到该模式。只有当URL路径为/users/show，并且HttpRequest.url_for()路由名称show_users调用该函数时，路由才会匹配，它将生成具有相同路径的URL。

## 应用程序谓词和虚拟主机

您可以将谓词看作一个接受请求对象引用并返回true或false的简单函数。形式上，谓词是实现[`Predicate`](https://docs.rs/actix-web/0.6.10/actix_web/pred/trait.Predicate.html) trait的任何对象 。Actix提供了几个谓词，你可以检查 API文档的[functions section](https://docs.rs/actix-web/0.6.10/actix_web/pred/index.html#functions)部分。

任何这些谓词都可以用于App::filter()方法。提供的谓词之一是Host，它可以根据请求的主机信息用作应用程序的过滤器。

{{< include-example example="application" file="vh.rs" section="vh" >}}
