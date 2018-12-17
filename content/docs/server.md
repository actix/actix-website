---
title: Server
menu: docs_basics
weight: 150
---

# The HTTP Server

The [**HttpServer**](../../actix-web/actix_web/server/struct.HttpServer.html) type is responsible for
serving http requests.

`HttpServer` accepts an application factory as a parameter, and the
application factory must have `Send` + `Sync` boundaries. More about that in the
*multi-threading* section.

To bind to a specific socket address,
[`bind()`](../../actix-web/actix_web/server/struct.HttpServer.html#method.bind)
must be used, and it may be called multiple times. To bind ssl socket,
[`bind_ssl()`](../../actix-web/actix_web/server/struct.HttpServer.html#method.bind_ssl)
or [`bind_tls()`](../../actix-web/actix_web/server/struct.HttpServer.html#method.bind_tls)
should be used. To start the http server, use one of the start methods.

- use [`start()`](../../actix-web/actix_web/server/struct.HttpServer.html#method.start)
for a server

`HttpServer` is an actix actor. It must be initialized within a properly
configured actix system:

{{< include-example example="server" section="main" >}}

> It is possible to start a server in a separate thread with the `run()` method. In that
> case the server spawns a new thread and creates a new actix system in it. To stop
> this server, send a `StopServer` message.

`HttpServer` is implemented as an actix actor. It is possible to communicate with the server
via a messaging system. Start method, e.g. `start()`, returns the
address of the started http server. It accepts several messages:

- `PauseServer` - Pause accepting incoming connections
- `ResumeServer` - Resume accepting incoming connections
- `StopServer` - Stop incoming connection processing, stop all workers and exit

{{< include-example example="server" file="signals.rs" section="signals" >}}

## Multi-threading

`HttpServer` automatically starts a number of http workers, by default
this number is equal to number of logical CPUs in the system. This number
can be overridden with the
[`HttpServer::workers()`](../../actix-web/actix_web/server/struct.HttpServer.html#method.workers) method.

{{< include-example example="server" file="workers.rs" section="workers" >}}

The server creates a separate application instance for each created worker. Application state
is not shared between threads. To share state, `Arc` could be used.

> Application state does not need to be `Send` and `Sync`,
> but factories must be `Send` + `Sync`.

## SSL

There are two features for ssl server: `tls` and `alpn`. The `tls` feature is
for `native-tls` integration and `alpn` is for `openssl`.

```toml
[dependencies]
actix-web = { version = "{{< actix-version "actix-web" >}}", features = ["alpn"] }
```

{{< include-example example="server" file="ssl.rs" section="ssl" >}}

> **Note**: the *HTTP/2.0* protocol requires
> [tls alpn](https://tools.ietf.org/html/rfc7301).
> At the moment, only `openssl` has `alpn` support.
> For a full example, check out
> [examples/tls](https://github.com/actix/examples/tree/master/tls).

To create the key.pem and cert.pem use the command. **Fill in your own subject**
```bash
$ openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
  -days 365 -sha256 -subj "/C=CN/ST=Fujian/L=Xiamen/O=TVlinux/OU=Org/CN=muro.lxd"
```
To remove the password, then copy nopass.pem to key.pem 
```bash
$ openssl rsa -in key.pem -out nopass.pem
```

## Keep-Alive

Actix can wait for requests on a keep-alive connection.

> *keep alive* connection behavior is defined by server settings.

- `75`, `Some(75)`, `KeepAlive::Timeout(75)` - enable 75 second *keep alive* timer.
- `None` or `KeepAlive::Disabled` - disable *keep alive*.
- `KeepAlive::Tcp(75)` - use `SO_KEEPALIVE` socket option.

{{< include-example example="server" file="ka.rs" section="ka" >}}

If the first option is selected, then *keep alive* state is
calculated based on the response's *connection-type*. By default
`HttpResponse::connection_type` is not defined. In that case *keep alive* is
defined by the request's http version.

> *keep alive* is **off** for *HTTP/1.0* and is **on** for *HTTP/1.1* and *HTTP/2.0*.

*Connection type* can be changed with `HttpResponseBuilder::connection_type()` method.

{{< include-example example="server" file="ka_tp.rs" section="example" >}}

## Graceful shutdown

`HttpServer` supports graceful shutdown. After receiving a stop signal, workers
have a specific amount of time to finish serving requests. Any workers still alive after the
timeout are force-dropped. By default the shutdown timeout is set to 30 seconds.
You can change this parameter with the
[`HttpServer::shutdown_timeout()`](../../actix-web/actix_web/server/struct.HttpServer.html#method.shutdown_timeout) method.

You can send a stop message to the server with the server address and specify if you want
graceful shutdown or not. The
[`start()`](../../actix-web/actix_web/server/struct.HttpServer.html#method.start)
method returns address of the server.

`HttpServer` handles several OS signals. *CTRL-C* is available on all OSs,
other signals are available on unix systems.

- *SIGINT* - Force shutdown workers
- *SIGTERM* - Graceful shutdown workers
- *SIGQUIT* - Force shutdown workers

> It is possible to disable signal handling with
> [`HttpServer::disable_signals()`](../../actix-web/actix_web/server/struct.HttpServer.html#method.disable_signals)
> method.
