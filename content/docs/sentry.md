---
title: Sentry
menu: docs_patterns
weight: 1020
---

# Sentry Crash Reporting

[Sentry](https://sentry.io/) is a crash reporting system that supports the
failure crate which is the base of the actix error reporting.  With a
middleware it's possible to automatically report server errors to sentry.

# Middleware

This middleware captures any error in the server error range (500 - 599)
and sends the attached error to sentry with its stacktrace.

```rust
use actix_web::{HttpRequest, HttpResponse, Error};
use actix_web::middleware::{Middleware, Response};
use failure::Fail;
use sentry::with_client_and_scope;
use sentry::protocol::{Event, Level};
use sentry::integrations::failure::exception_from_single_fail;

/// Reports certain failures to sentry.
pub struct CaptureSentryError;

impl<S> Middleware<S> for CaptureSentryError {
    fn response(&self, _: &mut HttpRequest<S>, mut resp: HttpResponse)
        -> Result<Response, Error>
    {
        if resp.status().is_server_error() {
            if let Some(error) = resp.error() {
                report_actix_error_to_sentry(error);
            }
        }
        Ok(Response::Done(resp))
    }
}

pub fn report_actix_error_to_sentry(err: &Error) {
    with_client_and_scope(|client, scope| {
        let mut exceptions = vec![
            exception_from_single_fail(err.cause(), Some(err.backtrace())),
        ];
        let mut ptr: Option<&Fail> = err.cause().cause();
        while let Some(cause) = ptr {
            exceptions.push(exception_from_single_fail(cause, cause.backtrace()));
            ptr = Some(cause);
        }
        exceptions.reverse();
        client.capture_event(
            Event {
                exceptions: exceptions,
                level: Level::Error,
                ..Default::default()
            },
            Some(scope),
        )
    });
}
```

# Middleware Usage

To use the middleware the [sentry crate](https://crates.io/crates/sentry) needs to be
initialized and configured.  Additionally it makes sense to also register the panic handler
to be informed about hard panics.

```rust
extern crate sentry;

use std::env;

fn main() {
    sentry::init("SENTRY_DSN_GOES_HERE");
    env::set_var("RUST_BACKTRACE", "1");
    sentry::integrations::panic::register_panic_handler();

    let mut app = App::with_state(state)
        .middleware(CaptureSentryError)
        // ...
}
```
