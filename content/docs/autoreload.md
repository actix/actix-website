---
title: Autoreloading
menu: docs_patterns
weight: 1000
---

# Auto-Reloading Development Server

During development it can be very handy to have cargo automatically recompile the code
on change.  This can be accomplished by using [cargo-watch][cargowatch].  Because an
actix app will typically bind to a port for listening for incoming HTTP requests it makes
sense to combine this with the [listenfd][listenfd] crate and the [systemfd][systemfd]
utility to ensure the socket is kept open while the app is compiling and reloading.

`systemfd` will open a socket and pass it to `cargo-watch` which will watch for
changes and then invoke the compiler and run your actix app.  The actix app
will then use `listenfd` to pick up the socket that `systemfd` opened.

## Binaries Necessary

For an automatic reloading experience you need to install `cargo-watch` and
`systemfd`.  Both are written in Rust and can be installed with `cargo install`:

```
cargo install systemfd cargo-watch
```

## Code Changes

Additionally you need to slightly modify your actix app so that it can pick up
an external socket opened by `systemfd`.  Add the listenfd dependency to your app:

```ini
[dependencies]
listenfd = "0.3"
```

Then modify your server code to only invoke `bind` as a fallback:

{{< include-example example="autoreload" file="main.rs" section="autoreload" >}}

## Running the Server

To now run the development server invoke this command:

```
systemfd --no-pid -s http::3000 -- cargo watch -x run
```

[cargowatch]: https://github.com/passcod/cargo-watch
[listenfd]: https://crates.io/crates/listenfd
[systemfd]: https://github.com/mitsuhiko/systemfd
