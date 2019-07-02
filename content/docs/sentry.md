---
title: Sentry
menu: docs_patterns
weight: 1020
---

# Sentry Crash Reporting

{{% alert %}}
NOTE: Sentry currently does not work with `actix-web` 1.0. Please checkout this
[issue](https://github.com/getsentry/sentry-rust/issues/143) for more details.
{{% /alert %}}

[Sentry][sentrysite] is a crash reporting system that supports the failure crate which
is the base of the actix error reporting.  With a middleware it's possible to
automatically report server errors to Sentry.

# Middleware

This middleware captures any error in the server error range (500 - 599)
and sends the attached error to sentry with its stacktrace.

To use the middleware the [sentry crate][sentrycrate] needs to be initialized and configured
and the [sentry-actix middleware][sentrymiddleware] needs to be added.  Additionally it
makes sense to also register the panic handler to be informed about hard panics.

{{< include-example example="sentry" file="main.rs" section="middleware" >}}

# Reusing the Hub

If you use this integration the default sentry hub (`Hub::current()`) is typically the wrong one.
To get the request specific one you need to use the `ActixWebHubExt` trait:

{{< include-example example="sentry" file="main.rs" section="hub" >}}

The hub can also be made current for the duration of a call.  Then `Hub::current()` works correctly
until the end of the `run` block.

{{< include-example example="sentry" file="main.rs" section="hub2" >}}

[sentrysite]: https://sentry.io/
[sentrycrate]: https://crates.io/crates/sentry
[sentrymiddleware]: https://crates.io/crates/sentry-actix
