---
title: Handlers
menu: docs_basics
weight: 160
---

# Request Handlers

A request handler can be any object that implements
[*Handler*](../../actix-web/actix_web/dev/trait.Handler.html) trait.

Request handling happens in two stages. First the handler object is called,
returning any object that implements the
[*Responder*](../../actix-web/actix_web/trait.Responder.html#foreign-impls) trait.
Then, `respond_to()` is called on the returned object, converting itself to a `AsyncResult` or `Error`.

By default actix provides `Responder` implementations for some standard types,
such as `&'static str`, `String`, etc.

> For a complete list of implementations, check
> [*Responder documentation*](../../actix-web/actix_web/trait.Responder.html#foreign-impls).

Examples of valid handlers:

```rust
fn index(req: &HttpRequest) -> &'static str {
    "Hello world!"
}
```

```rust
fn index(req: &HttpRequest) -> String {
    "Hello world!".to_owned()
}
```

You can also change the signature to return `impl Responder` which works well if more
complex types are involved.

```rust
fn index(req: &HttpRequest) -> impl Responder {
    Bytes::from_static("Hello world!")
}
```

```rust,ignore
fn index(req: &HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    ...
}
```

*Handler* trait is generic over *S*, which defines the application state's type.
Application state is accessible from the handler with the `HttpRequest::state()` method;
however, state is accessible as a read-only reference. If you need mutable access to state,
it must be implemented.

> **Note**: Alternatively, the handler can use interior mutably to access its own
> state. **Beware**, actix creates multiple copies
> of the application state and the handlers, unique for each thread. If you run your
> application in several threads, actix will create the same amount as number of threads
> of application state objects and handler objects.

Here is an example of a handler that stores the number of processed requests:

```rust
use actix_web::{App, HttpRequest, HttpResponse, dev::Handler};

struct MyHandler(Cell<usize>);

impl<S> Handler<S> for MyHandler {
    type Result = HttpResponse;

    /// Handle request
    fn handle(&self, req: &HttpRequest<S>) -> Self::Result {
        let i = self.0.get();
        self.0.set(i + 1);
        HttpResponse::Ok().into()
    }
}

fn main(){
    server::new(|| App::new()
                .resource("/", |r| r.h(MyHandler(Cell::new(0)))))  //use r.h() to bind handler, not the r.f()
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();
}
```

Although this handler will work, `self.0` will be different depending on the number of threads and
number of requests processed per thread. A proper implementation would use `Arc` and `AtomicUsize`.

```rust
use actix_web::{server, App, HttpRequest, HttpResponse, dev::Handler};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

struct MyHandler(Arc<AtomicUsize>);

impl<S> Handler<S> for MyHandler {
    type Result = HttpResponse;

    /// Handle request
    fn handle(&self, req: &HttpRequest<S>) -> Self::Result {
        self.0.fetch_add(1, Ordering::Relaxed);
        HttpResponse::Ok().into()
    }
}

fn main() {
    let sys = actix::System::new("example");

    let inc = Arc::new(AtomicUsize::new(0));

    server::new(
        move || {
            let cloned = inc.clone();
            App::new()
                .resource("/", move |r| r.h(MyHandler(cloned)))
        })
        .bind("127.0.0.1:8088").unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
}
```

> Be careful with synchronization primitives like `Mutex` or `RwLock`. The `actix-web` framework
> handles requests asynchronously. By blocking thread execution, all concurrent
> request handling processes would block. If you need to share or update some state
> from multiple threads, consider using the [actix](https://actix.github.io/actix/actix/) actor system.

## Response with custom type

To return a custom type directly from a handler function, the type needs to implement the `Responder` trait.

Let's create a response for a custom type that serializes to an `application/json` response:

```rust
# extern crate actix;
# extern crate actix_web;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
use actix_web::{server, App, HttpRequest, HttpResponse, Error, Responder, http};

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

/// Responder
impl Responder for MyObj {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;

        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

fn index(req: &HttpRequest) -> impl Responder {
    MyObj { name: "user" }
}

fn main() {
    let sys = actix::System::new("example");

    server::new(
        || App::new()
            .resource("/", |r| r.method(http::Method::GET).f(index)))
        .bind("127.0.0.1:8088").unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
}
```

## Async handlers

There are two different types of async handlers. Response objects can be generated asynchronously
or more precisely, any type that implements the [*Responder*](../../actix-web/actix_web/trait.Responder.html) trait.

In this case, the handler must return a `Future` object that resolves to the *Responder* type, i.e:

```rust
use actix_web::*;
use bytes::Bytes;
use futures::stream::once;
use futures::future::{Future, result};

fn index(req: &HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {

    result(Ok(HttpResponse::Ok()
              .content_type("text/html")
              .body(format!("Hello!"))))
           .responder()
}

fn index2(req: &HttpRequest) -> Box<Future<Item=&'static str, Error=Error>> {
    result(Ok("Welcome!"))
        .responder()
}

fn main() {
    App::new()
        .resource("/async", |r| r.route().a(index))
        .resource("/", |r| r.route().a(index2))
        .finish();
}
```

Or the response body can be generated asynchronously. In this case, body
must implement the stream trait `Stream<Item=Bytes, Error=Error>`, i.e:

```rust
use actix_web::*;
use bytes::Bytes;
use futures::stream::once;

fn index(req: &HttpRequest) -> HttpResponse {
    let body = once(Ok(Bytes::from_static(b"test")));

    HttpResponse::Ok()
       .content_type("application/json")
       .body(Body::Streaming(Box::new(body)))
}

fn main() {
    App::new()
        .resource("/async", |r| r.f(index))
        .finish();
}
```

Both methods can be combined. (i.e Async response with streaming body)

It is possible to return a `Result` where the `Result::Item` type can be `Future`.
In this example, the `index` handler can return an error immediately or return a
future that resolves to a `HttpResponse`.

```rust
use actix_web::*;
use bytes::Bytes;
use futures::stream::once;
use futures::future::{Future, result};

fn index(req: &HttpRequest) -> Result<Box<Future<Item=HttpResponse, Error=Error>>, Error> {
    if is_error() {
       Err(error::ErrorBadRequest("bad request"))
    } else {
       Ok(Box::new(
           result(Ok(HttpResponse::Ok()
                  .content_type("text/html")
                  .body(format!("Hello!"))))))
    }
}
```

## Different return types (Either)

Sometimes, you need to return different types of responses. For example,
you can error check and return errors, return async responses, or any result that requires two different types.

For this case, the [*Either*](../../actix-web/actix_web/enum.Either.html) type can be used.
`Either` allows combining two different responder types into a single type.

```rust
use futures::future::{Future, result};
use actix_web::{Either, Error, HttpResponse};

type RegisterResult = Either<HttpResponse, Box<Future<Item=HttpResponse, Error=Error>>>;

fn index(req: &HttpRequest) -> RegisterResult {
    if is_a_variant() { // <- choose variant A
        Either::A(
            HttpResponse::BadRequest().body("Bad data"))
    } else {
        Either::B(      // <- variant B
            result(Ok(HttpResponse::Ok()
                   .content_type("text/html")
                   .body(format!("Hello!")))).responder())
    }
}
```
