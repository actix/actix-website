---
title: Autoreloading
menu: docs_patterns
weight: 1000
---

# Auto-Reloading Development Server

During development it can be very handy to have cargo automatically recompile
the code on change.  This can be accomplished by using
[cargo-watch](https://github.com/passcod/cargo-watch).  Because an actix app
will typically bind to a port for listening for incoming HTTP requests it makes
sense to combine this with the [listenfd](https://crates.io/crates/listenfd)
crate and the [systemfd](https://github.com/mitsuhiko/systemfd) utility to
ensure the socket is kept open while the app is compiling and reloading.

`systemfd` will open a socket and pass it to `cargo-watch` which will watch for
changes and then invoke the compiler and run your actix app.  The actix app
will then use `listenfd` to pick up the socket that `systemfd` opened.

## Binaries Necessary

For an automatic reloading experience you need to install `cargo-watch` and
`systemfd`.  Both are written in rust and can be installed with `cargo install`:

```
cargo install systemfd cargo-watch
```

## Code Changes

Additionally you need to slightly modify your actix app so that it can pick up
an external socket opened by `systemfd`.  Add the listenfd dependency to your
app:

```ini
[dependencies]
listenfd = "0.3"
```

Then modify your server code to only invoke `bind` as a fallback:

```rust
extern crate actix_web;
extern crate listenfd;

use listenfd::ListenFd;
use actix_web::{server, App, HttpRequest, Responder};

fn index(_req: &HttpRequest) -> impl Responder {
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

## Running the Server

To now run the development server invoke this command:

```
systemfd --no-pid -s http::3000 -- cargo watch -x run
```
