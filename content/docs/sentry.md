---
title: Sentry
menu: docs_patterns
weight: 1020
---

# Sentry Crash Reporting

[Sentry](https://sentry.io/) is a crash reporting system that supports the
failure crate which is the base of the actix error reporting.  With a
middleware it's possible to automatically report server errors to Sentry.

# Middleware

This middleware captures any error in the server error range (500 - 599)
and sends the attached error to sentry with its stacktrace.

To use the middleware the [sentry crate](https://crates.io/crates/sentry) needs to be
initialized and configured and the [sentry-actix middleware](https://crates.io/crates/sentry-actix)
needs to be added.  Additionally it makes sense to also register the panic handler
to be informed about hard panics.

```rust
extern crate sentry;
extern crate sentry_actix;

use sentry_actix::SentryMiddleware;

use std::env;

fn main() {
    let _guard = sentry::init("SENTRY_DSN_GOES_HERE");
    env::set_var("RUST_BACKTRACE", "1");
    sentry::integrations::panic::register_panic_handler();

    let mut app = App::with_state(state)
        .middleware(SentryMiddleware::new())
        // ...
}
```

# Reusing the Hub

If you use this integration the default sentry hub (`Hub::current()`) is typically the wrong one.
To get the request specific one you need to use the `ActixWebHubExt` trait:

```rust
use sentry::{Hub, Level};
use sentry_actix::ActixWebHubExt;

let hub = Hub::from_request(req);
hub.capture_message("Something is not well", Level::Warning);
```

The hub can also be made current for the duration of a call.  Then `Hub::current()` works correctly
until the end of the `run` block.

```rust
use sentry::{Hub, Level};
use sentry_actix::ActixWebHubExt;

let hub = Hub::from_request(req);
Hub::run(hub, || {
    sentry::capture_message("Something is not well", Level::Warning);
});
```
