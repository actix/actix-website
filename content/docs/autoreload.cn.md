---
title: 自动重加载
menu: docs_patterns
weight: 1000
---

# 自动重新加载开发服务器

在开发过程中，cargo自动重新编译变更代码会非常方便。这可以通过使用[cargo-watch](https://github.com/passcod/cargo-watch)来完成 。由于actix应用程序通常会绑定到端口以侦听传入的HTTP请求，因此将它与[listenfd](https://crates.io/crates/listenfd)和[systemfd](https://github.com/mitsuhiko/systemfd)实用程序结合起来以确保套接字在应用程序编译和重新加载时保持打开状态是有意义的。

`systemfd`将打开一个套接字并将其传递给`cargo-watch`以监视更改，然后调用编译器并运行您的actix应用程序。actix应用程序将使用`listenfd`获取 `systemfd`打开的套接字systemfd。

## 需要的二进制文件

对于自动重新加载体验，您需要安装`cargo-watch`和 `systemfd`。两者都用`cargo install`安装

```
cargo install systemfd cargo-watch
```

## 修改代码

此外，您需要稍微修改您的actix应用程序，以便它可以获取由`systemfd`打开的外部套接字。将`listenfd`添加到您的应用依赖项中：

```ini
[dependencices]
listenfd = "0.3"
```

然后修改您的服务器代码以仅以`bind`作为回调：

```rust
extern crate listenfd;

use listenfd::ListenFd;
use actix_web::{server, App, HttpRequest, Responder};

fn index(_req: HttpRequest) -> impl Responder {
    "Hello World!"
}

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::new()
            .resource("/", |r| r.f(index))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:3000").unwrap()
    };

    server.run();
}
```

## 运行服务器

现在运行开发服务器调用这个命令：

```
systemfd --no-pid -s http::3000 -- cargo watch -x run
```
