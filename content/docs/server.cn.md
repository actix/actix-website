---
title: 服务器
menu: docs_basics
weight: 150
---

# HTTP服务器

该[**HttpServer**](../../actix-web/actix_web/server/struct.HttpServer.html)类型负责服务的HTTP请求。

`HttpServer`接受应用程序工厂作为参数，并且应用程序工厂必须具有Send+ Sync边界。p
要绑定到特定的套接字地址， [`bind()`](../../actix-web/actix_web/server/struct.HttpServer.html#method.bind) 必须使用，并且可能会多次调用它。绑定ssl套接字使用[`bind_ssl()`](../../actix-web/actix_web/server/struct.HttpServer.html#method.bind_ssl)或[`bind_tls()`](../../actix-web/actix_web/server/struct.HttpServer.html#method.bind_tls)。启动http服务器，启动方法之一是：

- use [`start()`](https://actix.rs/actix-web/actix_web/server/struct.HttpServer.html#method.start)
for a server

`HttpServer`是一位actix actor。它必须在正确配置的actix系统中初始化：

{{< include-example example="server" section="main" >}}

> 可以使用该run()方法在单独的线程中启动服务器。在这种情况下，服务器会产生一个新线程并在其中创建一个新的actix系统。要停止此服务器，请发送`StopServer`消息。

`HttpServer`被实施为actix actor。可以通过消息传递系统与服务器进行通信。启动方法，例如`start()`，返回启动的http服务器的地址。它接受几种消息：

- PauseServer - 暂停接受传入连接
- ResumeServer - 继续接受传入连接
- StopServer - 停止传入连接处理，停止所有workers并退出

{{< include-example example="server" file="signals.rs" section="signals" >}}

## 多线程

`HttpServer`自动启动一些http worker，默认情况下这个数量等于系统中逻辑CPU的数量。该数量可以用该[`HttpServer::workers()`](../../actix-web/actix_web/server/struct.HttpServer.html#method.workers)方法覆盖 。


{{< include-example example="server" file="workers.rs" section="workers" >}}

服务器为每个创建的worker创建一个单独的应用实例。应用程序状态不在线程之间共享。分享状态，可以使用Arc。

>应用程序状态并不需要是Send和Sync，但是工厂必须是Send+ Sync。

## SSL

有两种功能的ssl服务器：`tls`和`alpn`。该tls功能由native-tls集成，alpn由openssl。

```toml
[dependencies]
actix-web = { version = "{{< actix-version "actix-web" >}}", features = ["alpn"] }
```

{{< include-example example="server" file="ssl.rs" section="ssl" >}}

》 **注意**：HTTP / 2.0协议需要[tls alpn](https://tools.ietf.org/html/rfc7301)。目前，只有openssl有alpn支持。完整示例，请查看[examples/tls](https://github.com/actix/examples/tree/master/tls).

要创建key.pem和cert.pem，请使用以下命令。**Fill in your own subject**

```bash
$ openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
  -days 365 -sha256 -subj "/C=CN/ST=Fujian/L=Xiamen/O=TVlinux/OU=Org/CN=muro.lxd"
```

要删除密码，请将nopass.pem复制到key.pem

```bash
$ openssl rsa -in key.pem -out nopass.pem
```

## Keep-Alive

Actix可以等待keep-alive的请求。

》 *keep-alive*连接行为由服务器设置定义。

- `75`, `Some(75)`, `KeepAlive::Timeout(75)` - 75秒keep alive定时器。
- `None` or `KeepAlive::Disabled` - 禁用 *keep alive*.
- `KeepAlive::Tcp(75)` -  使用 `SO_KEEPALIVE` socket 选项.

{{< include-example example="server" file="ka.rs" section="ka" >}}

如果选择第一个选项，则*keep alive*状态根据响应的*connection-type*计算。默认情况下`HttpResponse::connection_type`未定义。在这种情况下， *keep alive* 状态由请求的http版本定义。

> *keep alive* 是 **关闭** 对于 *HTTP/1.0* 然而是 **打开** 对于 *HTTP/1.1* 和 *HTTP/2.0*.

*Connection type*可以用`HttpResponseBuilder::connection_type()`方法改变。

{{< include-example example="server" file="ka_tp.rs" section="example" >}}

## 优雅的关机

`HttpServer`支持优雅的关机。收到停止信号后，workers会有特定的时间完成服务请求。任何在超时后仍然活着的workers（工作线程）都会被迫停止。默认情况下，关机超时设置为30秒。您可以使用[`HttpServer::shutdown_timeout()`](../../actix-web/actix_web/server/struct.HttpServer.html#method.shutdown_timeout)方法更改此参数 。

您可以使用服务器地址向服务器发送停止消息，并指定是否要进行正常关机。[`start()`](../../actix-web/actix_web/server/struct.HttpServer.html#method.start)方法返回服务器的地址。

`HttpServer`处理几个OS信号。所有操作系统都提供CTRL-C，其他信号在unix系统上可用。

- *SIGINT* - 强制关闭工作线程
- *SIGTERM* - 优雅的停止工作线程
- *SIGQUIT* - 制关闭 workers工作线程

> 可以用[`HttpServer::disable_signals()`](../../actix-web/actix_web/server/struct.HttpServer.html#method.disable_signals)
方法禁用信号处理 。
