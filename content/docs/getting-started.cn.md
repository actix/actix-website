---
title: 开始
menu: docs_basics
weight: 130
---

# 开始

我们来编写第一个actix web应用程序！

## Hello, world!

首先创建一个新的基于二进制的Cargo项目并进入新目录：

```bash
cargo new hello-world
cd hello-world
```

现在，确保actix-web的Cargo.toml 包含以下项目依赖关系：

```ini
[dependencies]
actix-web = "{{< actix-version "actix-web" >}}"
```
为了实现一个Web服务器，我们首先需要创建一个请求处理程序。请求处理函数接受一个HttpRequest实例作为其唯一参数，并返回一个可转换为HttpResponse的类型：


文件名: `src/main.rs`

{{< include-example example="getting-started" section="setup" >}}

接下来，创建一个Application实例，并将请求处理程序与应用程序的resource一起注册在特定HTTP方法和路径，然后，应用程序实例可以用于HttpServer来侦听将传入的连接。服务器接受一个应该返回一个HttpHandler实例的函数 。简单来说server::new可以使用了，它是HttpServer::new的简写：

{{< include-example example="getting-started" section="main" >}}


仅此而已！现在编译并运行该程序cargo run。去http://localhost:8088 看结果。

如果你想要在开发过程中重新编译后自动重新加载服务器。请查看[自动重新加载模式](../autoreload/)。
