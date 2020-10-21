---
title: Getting Started
menu: docs_basics
weight: 130
---

## Installing Rust

If you don't have Rust yet, we recommend you use `rustup` to manage your Rust installation. The
[official rust guide][rustguide] has a wonderful section on getting started.

Actix Web currently has a minimum supported Rust version (MSRV) of {{< rust-version "actix-web" >}}.
Running `rustup update` will ensure you have the latest and greatest Rust version available. As
such, this guide assumes you are running Rust {{< rust-version "actix-web" >}} or later.

## Hello, world!

Start by creating a new binary-based Cargo project and changing into the new directory:

```bash
cargo new hello-world
cd hello-world
```

Add `actix-web` as a dependency of your project by adding the following to your `Cargo.toml` file.

```toml
[dependencies]
actix-web = "{{< actix-version "actix-web" >}}"
```

Request handlers use async functions that accept zero or more parameters. These parameters can be
extracted from a request (see `FromRequest` trait) and returns a type that can be converted into an
`HttpResponse` (see `Responder` trait):

{{< include-example example="getting-started" section="handlers" >}}

Notice that some of these handlers have routing information attached directly using the built-in
macros. These allow you to specify the method and path that the handler should respond to. You will
see below how to register the other route that does not use a routing macro.

Next, create an `App` instance and register the request handlers. Use `App::service` for the
handlers using routing macros and `App::route` for manually routed handlers, declaring the a path
and method. Finally, the app is started inside an `HttpServer` which will serve incoming requests
using your `App` as an "application factory".

{{< include-example example="getting-started" section="main" >}}

That's it! Compile and run the program with `cargo run`. The `#[actix_web::main]` macro executes the
async main function within the actix runtime. Now you can go to `http://localhost:8080/` or any of
the other routes you defined to see the results.

<!-- LINKS -->

[rustguide]: https://doc.rust-lang.org/book/ch01-01-installation.html
[actix-web-codegen]: https://docs.rs/actix-web-codegen/
