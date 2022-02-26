---
title: Server
menu: docs_basics
weight: 150
---

# The HTTP Server

The [**HttpServer**][httpserverstruct] type is responsible for serving HTTP requests.

`HttpServer` accepts an application factory as a parameter, and the application factory must have `Send` + `Sync` boundaries. More about that in the _multi-threading_ section.

To bind to a specific socket address, [`bind()`][bindmethod] must be used, and it may be called multiple times. To bind ssl socket, [`bind_openssl()`][bindopensslmethod] or [`bind_rustls()`][bindrusttls] should be used. To run the HTTP server, use the `HttpServer::run()` method.

{{< include-example example="server" section="main" >}}

The `run()` method returns an instance of the [`Server`][server] type. Methods of server type could be used for managing the HTTP server

- `pause()` - Pause accepting incoming connections
- `resume()` - Resume accepting incoming connections
- `stop()` - Stop incoming connection processing, stop all workers and exit

The following example shows how to start the HTTP server in a separate thread.

{{< include-example example="server" file="signals.rs" section="signals" >}}

## Multi-threading

`HttpServer` automatically starts a number of HTTP _workers_, by default this number is equal to the number of logical CPUs in the system. This number can be overridden with the [`HttpServer::workers()`][workers] method.

{{< include-example example="server" file="workers.rs" section="workers" >}}

Once the workers are created, they each receive a separate _application_ instance to handle requests. Application state is not shared between the threads, and handlers are free to manipulate their copy of the state with no concurrency concerns.

> Application state does not need to be `Send` or `Sync`, but application factories must be `Send` + `Sync`.

To share state between worker threads, use an `Arc`. Special care should be taken once sharing and synchronization are introduced. In many cases, performance costs are inadvertently introduced as a result of locking the shared state for modifications.

In some cases these costs can be alleviated using more efficient locking strategies, for example using [read/write locks](https://doc.rust-lang.org/std/sync/struct.RwLock.html) instead of [mutexes](https://doc.rust-lang.org/std/sync/struct.Mutex.html) to achieve non-exclusive locking, but the most performant implementations often tend to be ones in which no locking occurs at all.

Since each worker thread processes its requests sequentially, handlers which block the current thread will cause the current worker to stop processing new requests:

```rust
fn my_handler() -> impl Responder {
    std::thread::sleep(Duration::from_secs(5)); // <-- Bad practice! Will cause the current worker thread to hang!
    "response"
}
```

For this reason, any long, non-cpu-bound operation (e.g. I/O, database operations, etc.) should be expressed as futures or asynchronous functions. Async handlers get executed concurrently by worker threads and thus don't block execution:

```rust
async fn my_handler() -> impl Responder {
    tokio::time::delay_for(Duration::from_secs(5)).await; // <-- Ok. Worker thread will handle other requests here
    "response"
}
```

The same limitation applies to extractors as well. When a handler function receives an argument which implements `FromRequest`, and that implementation blocks the current thread, the worker thread will block when running the handler. Special attention must be given when implementing extractors for this very reason, and they should also be implemented asynchronously where needed.

## SSL

There are two features for the ssl server: `rustls` and `openssl`. The `rustls` feature is for `rustls` integration and `openssl` is for `openssl`.

```toml
[dependencies]
actix-web = { version = "{{< actix-version "actix-web" >}}", features = ["openssl"] }
openssl = { version = "0.10" }
```

{{< include-example example="server" file="ssl.rs" section="ssl" >}}

> **Note**: the _HTTP/2.0_ protocol requires [tls alpn][tlsalpn]. At the moment, only `openssl` has `alpn` support. For a full example, check out [examples/openssl][exampleopenssl].

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

> _keep alive_ connection behavior is defined by server settings.

- `75`, `Some(75)`, `KeepAlive::Timeout(75)` - enable 75 second _keep alive_ timer.
- `None` or `KeepAlive::Disabled` - disable _keep alive_.
- `KeepAlive::Tcp(75)` - use `SO_KEEPALIVE` socket option.

{{< include-example example="server" file="keep_alive.rs" section="keep-alive" >}}

If the first option above is selected, then _keep alive_ state is calculated based on the response's _connection-type_. By default `HttpResponse::connection_type` is not defined. In that case _keep alive_ is defined by the request's HTTP version.

> _keep alive_ is **off** for _HTTP/1.0_ and is **on** for _HTTP/1.1_ and _HTTP/2.0_.

_Connection type_ can be changed with `HttpResponseBuilder::connection_type()` method.

{{< include-example example="server" file="keep_alive_tp.rs" section="example" >}}

## Graceful shutdown

`HttpServer` supports graceful shutdown. After receiving a stop signal, workers have a specific amount of time to finish serving requests. Any workers still alive after the timeout are force-dropped. By default the shutdown timeout is set to 30 seconds. You can change this parameter with the [`HttpServer::shutdown_timeout()`][shutdowntimeout] method.

`HttpServer` handles several OS signals. _CTRL-C_ is available on all OSs, other signals are available on unix systems.

- _SIGINT_ - Force shutdown workers
- _SIGTERM_ - Graceful shutdown workers
- _SIGQUIT_ - Force shutdown workers

> It is possible to disable signal handling with [`HttpServer::disable_signals()`][disablesignals] method.

[server]: https://docs.rs/actix-web/3/actix_web/dev/struct.Server.html
[httpserverstruct]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html
[bindmethod]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html#method.bind
[bindopensslmethod]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html#method.bind_openssl
[bindrusttls]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html#method.bind_rustls
[workers]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html#method.workers
[tlsalpn]: https://tools.ietf.org/html/rfc7301
[exampleopenssl]: https://github.com/actix/examples/tree/master/security/openssl
[shutdowntimeout]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html#method.shutdown_timeout
[disablesignals]: https://docs.rs/actix-web/3/actix_web/struct.HttpServer.html#method.disable_signals
