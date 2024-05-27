---
title: Server
---

import RenderCodeBlock from '@theme/CodeBlock';
import CodeBlock from '@site/src/components/code_block';
import vars from "@site/vars";

# The HTTP Server

The [**HttpServer**][httpserverstruct] type is responsible for serving HTTP requests.

`HttpServer` accepts an application factory as a parameter, and the application factory must have `Send` + `Sync` boundaries. More about that in the _multi-threading_ section.

To start the web server it must first be bound to a network socket. Use [`HttpServer::bind()`][bindmethod] with a socket address tuple or string such as `("127.0.0.1", 8080)` or `"0.0.0.0:8080"`. This will fail if the socket is being used by another application.

After the `bind` is successful, use [`HttpServer::run()`][httpserver_run] to return a [`Server`][server] instance. The `Server` must be `await`ed or `spawn`ed to start processing requests and will run until it receives a shutdown signal (such as, by default, a `ctrl-c`; [read more here](#graceful-shutdown)).

<CodeBlock example="server" section="main" />

## Multi-Threading

`HttpServer` automatically starts a number of HTTP _workers_, by default this number is equal to the number of physical CPUs in the system. This number can be overridden with the [`HttpServer::workers()`][workers] method.

<CodeBlock example="server" file="workers.rs" section="workers" />

Once the workers are created, they each receive a separate _application_ instance to handle requests. Application state is not shared between the threads, and handlers are free to manipulate their copy of the state with no concurrency concerns.

Application state does not need to be `Send` or `Sync`, but application factories must be `Send` + `Sync`.

To share state between worker threads, use an `Arc`/`Data`. Special care should be taken once sharing and synchronization are introduced. In many cases, performance costs are inadvertently introduced as a result of locking the shared state for modifications.

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
    tokio::time::sleep(Duration::from_secs(5)).await; // <-- Ok. Worker thread will handle other requests here
    "response"
}
```

The same limitation applies to extractors as well. When a handler function receives an argument which implements `FromRequest`, and that implementation blocks the current thread, the worker thread will block when running the handler. Special attention must be given when implementing extractors for this very reason, and they should also be implemented asynchronously where needed.

## TLS / HTTPS

Actix Web supports two TLS implementations out-of-the-box: `rustls` and `openssl`.

The `rustls` crate feature is for `rustls` integration and `openssl` is for `openssl` integration.

<!-- DEPENDENCY -->

<RenderCodeBlock className="language-toml">
{`[dependencies]
actix-web = { version = "${vars.actixWebMajorVersion}", features = ["openssl"] }
openssl = { version = "0.10" }
`}
</RenderCodeBlock>

<CodeBlock example="server" file="ssl.rs" section="ssl" />

To create the key.pem and cert.pem use the command. **Fill in your own subject**

```shell-session
$ openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
    -days 365 -sha256 -subj "/C=CN/ST=Fujian/L=Xiamen/O=TVlinux/OU=Org/CN=muro.lxd"
```

To remove the password, then copy nopass.pem to key.pem

```shell-session
$ openssl rsa -in key.pem -out nopass.pem
```

## Keep-Alive

Actix Web keeps connections open to wait for subsequent requests.

> _keep alive_ connection behavior is defined by server settings.

- `Duration::from_secs(75)` or `KeepAlive::Timeout(75)`: enables 75 second keep-alive timer.
- `KeepAlive::Os`: uses OS keep-alive.
- `None` or `KeepAlive::Disabled`: disables keep-alive.

<CodeBlock example="server" file="keep_alive.rs" section="keep-alive" />

If the first option above is selected, then keep-alive is enabled for HTTP/1.1 requests if the response does not explicitly disallow it by, for example, setting the [connection type][httpconnectiontype] to `Close` or `Upgrade`. Force closing a connection can be done with [the `force_close()` method on `HttpResponseBuilder`](https://docs.rs/actix-web/4/actix_web/struct.HttpResponseBuilder.html#method.force_close)

> Keep-alive is **off** for HTTP/1.0 and is **on** for HTTP/1.1 and HTTP/2.0.

<CodeBlock example="server" file="keep_alive_tp.rs" section="example" />

## Graceful shutdown

`HttpServer` supports graceful shutdown. After receiving a stop signal, workers have a specific amount of time to finish serving requests. Any workers still alive after the timeout are force-dropped. By default the shutdown timeout is set to 30 seconds. You can change this parameter with the [`HttpServer::shutdown_timeout()`][shutdowntimeout] method.

`HttpServer` handles several OS signals. _CTRL-C_ is available on all OSes, other signals are available on unix systems.

- _SIGINT_ - Force shutdown workers
- _SIGTERM_ - Graceful shutdown workers
- _SIGQUIT_ - Force shutdown workers

> It is possible to disable signal handling with [`HttpServer::disable_signals()`][disablesignals] method.

[server]: https://docs.rs/actix-web/4/actix_web/dev/struct.Server.html
[httpserverstruct]: https://docs.rs/actix-web/4/actix_web/struct.HttpServer.html
[bindmethod]: https://docs.rs/actix-web/4/actix_web/struct.HttpServer.html#method.bind
[httpserver_run]: https://docs.rs/actix-web/4/actix_web/struct.HttpServer.html#method.run
[bindopensslmethod]: https://docs.rs/actix-web/4/actix_web/struct.HttpServer.html#method.bind_openssl
[bindrusttls]: https://docs.rs/actix-web/4/actix_web/struct.HttpServer.html#method.bind_rustls
[workers]: https://docs.rs/actix-web/4/actix_web/struct.HttpServer.html#method.workers
[tlsalpn]: https://tools.ietf.org/html/rfc7301
[exampleopenssl]: https://github.com/actix/examples/tree/master/security/openssl
[shutdowntimeout]: https://docs.rs/actix-web/4/actix_web/struct.HttpServer.html#method.shutdown_timeout
[disablesignals]: https://docs.rs/actix-web/4/actix_web/struct.HttpServer.html#method.disable_signals
[httpconnectiontype]: https://docs.rs/actix-web/4/actix_web/http/enum.ConnectionType.html
