---
title: Getting Started
menu: docs_basics
weight: 130
---

# Getting Started

Let’s write our first `actix-web` application!

## Hello, world!

Start by creating a new binary-based Cargo project and changing into the new directory:

```bash
cargo new hello-world
cd hello-world
```

Now, add `actix-web` as dependencies of your project by ensuring your `Cargo.toml`
contains the following:

```ini
[dependencies]
actix-web = "{{< actix-version "actix-web" >}}"
```

In order to implement a web server, we first need to create a request handler.

A request handler is a function that accepts zero or more parameters that can be
extracted from a request (ie, `impl FromRequest`) and returns a type that can be
converted into an `HttpResponse` (ie, `impl Responder`):

{{< include-example example="getting-started" section="setup" >}}

Next, create an `App` instance and register the request handler with the application's
`route` on a *path* and with a particular *HTTP method*. After that, the application
instance can be used with `HttpServer` to listen for incoming connections. The server
accepts a function that should return an application factory.

{{< include-example example="getting-started" section="main" >}}

That's it! Now, compile and run the program with `cargo run`.
Head over to ``http://localhost:8088/`` to see the results.

If you want, you can have an automatic reloading server during development
that recompiles on demand.  To see how this can be accomplished have a look
at the [autoreload pattern](../autoreload/).
