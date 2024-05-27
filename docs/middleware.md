---
title: Middleware
---

import CodeBlock from "@site/src/components/code_block";

# Middleware

Actix Web's middleware system allows us to add additional behavior to request/response processing. Middleware can hook into an incoming request process, enabling us to modify requests as well as halt request processing to return a response early.

Middleware can also hook into response processing.

Typically, middleware is involved in the following actions:

- Pre-process the Request
- Post-process a Response
- Modify application state
- Access external services (redis, logging, sessions)

Middleware is registered for each `App`, `scope`, or `Resource` and executed in opposite order as registration. In general, a _middleware_ is a type that implements the [_Service trait_][servicetrait] and [_Transform trait_][transformtrait]. Each method in the traits has a default implementation. Each method can return a result immediately or a _future_ object.

The following demonstrates creating a simple middleware:

<CodeBlock example="middleware" file="main.rs" section="simple" />

Alternatively, for simple use cases, you can use [_wrap_fn_][wrap_fn] to create small, ad-hoc middleware:

<CodeBlock example="middleware" file="wrap_fn.rs" section="wrap-fn" />

> Actix Web provides several useful middleware, such as _logging_, _user sessions_, _compress_, etc.

**Warning: if you use `wrap()` or `wrap_fn()` multiple times, the last occurrence will be executed first.**

## Logging

Logging is implemented as a middleware. It is common to register a logging middleware as the first middleware for the application. Logging middleware must be registered for each application.

The `Logger` middleware uses the standard log crate to log information. You should enable logger for _actix_web_ package to see access log ([env_logger][envlogger] or similar).

### Usage

Create `Logger` middleware with the specified `format`. Default `Logger` can be created with `default` method, it uses the default format:

```ignore
  %a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T
```

<CodeBlock example="middleware" file="logger.rs" section="logger" />

The following is an example of the default logging format:

```log
INFO:actix_web::middleware::logger: 127.0.0.1:59934 [02/Dec/2017:00:21:43 -0800] "GET / HTTP/1.1" 302 0 "-" "curl/7.54.0" 0.000397
INFO:actix_web::middleware::logger: 127.0.0.1:59947 [02/Dec/2017:00:22:40 -0800] "GET /index.html HTTP/1.1" 200 0 "-" "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.13; rv:57.0) Gecko/20100101 Firefox/57.0" 0.000646
```

### Format

- `%%` The percent sign
- `%a` Remote IP-address (IP-address of proxy if using reverse proxy)
- `%t` Time when the request was started to process
- `%P` The process ID of the child that serviced the request
- `%r` First line of request
- `%s` Response status code
- `%b` Size of response in bytes, including HTTP headers
- `%T` Time taken to serve the request, in seconds with floating fraction in .06f format
- `%D` Time taken to serve the request, in milliseconds
- `%{FOO}i` request.headers['FOO']
- `%{FOO}o` response.headers['FOO']
- `%{FOO}e` os.environ['FOO']

## Default headers

To set default response headers, the `DefaultHeaders` middleware can be used. The _DefaultHeaders_ middleware does not set the header if response headers already contain a specified header.

<CodeBlock example="middleware" file="default_headers.rs" section="default-headers" />

## User sessions

Actix Web provides a general solution for session management. The [**actix-session**][actixsession] middleware can use multiple backend types to store session data.

> By default, only cookie session backend is implemented. Other backend implementations can be added.

[**CookieSession**][cookiesession] uses cookies as session storage. `CookieSessionBackend` creates sessions which are limited to storing fewer than 4000 bytes of data, as the payload must fit into a single cookie. An internal server error is generated if a session contains more than 4000 bytes.

A cookie may have a security policy of _signed_ or _private_. Each has a respective `CookieSession` constructor.

A _signed_ cookie may be viewed but not modified by the client. A _private_ cookie may neither be viewed nor modified by the client.

The constructors take a key as an argument. This is the private key for cookie session - when this value is changed, all session data is lost.

In general, you create a `SessionStorage` middleware and initialize it with specific backend implementation, such as a `CookieSession`. To access session data the [`Session`][requestsession] extractor must be used. This method returns a [_Session_][sessionobj] object, which allows us to get or set session data.

> `actix_session::storage::CookieSessionStore` is available on the crate feature "cookie-session".

<CodeBlock example="middleware" file="user_sessions.rs" section="user-session" />

## Error handlers

`ErrorHandlers` middleware allows us to provide custom handlers for responses.

You can use the `ErrorHandlers::handler()` method to register a custom error handler for a specific status code. You can modify an existing response or create a completly new one. The error handler can return a response immediately or return a future that resolves into a response.

<CodeBlock example="middleware" file="errorhandler.rs" section="error-handler" />

[sessionobj]: https://docs.rs/actix-session/0.7/actix_session/struct.Session.html
[requestsession]: https://docs.rs/actix-session/0.7/actix_session/struct.Session.html
[cookiesession]: https://docs.rs/actix-session/0.7/actix_session/storage/struct.CookieSessionStore.html
[actixsession]: https://docs.rs/actix-session/0.7/actix_session/
[envlogger]: https://docs.rs/env_logger/*/env_logger/
[servicetrait]: https://docs.rs/actix-web/4/actix_web/dev/trait.Service.html
[transformtrait]: https://docs.rs/actix-web/4/actix_web/dev/trait.Transform.html
[wrap_fn]: https://docs.rs/actix-web/4/actix_web/struct.App.html#method.wrap_fn
