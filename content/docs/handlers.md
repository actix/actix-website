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

{{< include-example example="request-handlers" file="main.rs" section="main" >}}

Although this handler will work, `self.0` will be different depending on the number of threads and
number of requests processed per thread. A proper implementation would use `Arc` and `AtomicUsize`.

{{< include-example example="request-handlers" file="handlers_arc.rs" section="arc" >}}

> Be careful with synchronization primitives like `Mutex` or `RwLock`. The `actix-web` framework
> handles requests asynchronously. By blocking thread execution, all concurrent
> request handling processes would block. If you need to share or update some state
> from multiple threads, consider using the [actix](https://actix.github.io/actix/actix/) actor system.

## Response with custom type

To return a custom type directly from a handler function, the type needs to implement the `Responder` trait.

Let's create a response for a custom type that serializes to an `application/json` response:

{{< include-example example="responder-trait" file="main.rs" section="main" >}}

## Async handlers

There are two different types of async handlers. Response objects can be generated asynchronously
or more precisely, any type that implements the [*Responder*](../../actix-web/actix_web/trait.Responder.html) trait.

In this case, the handler must return a `Future` object that resolves to the *Responder* type, i.e:

{{< include-example example="async-handlers" file="main.rs" section="main" >}}

Or the response body can be generated asynchronously. In this case, body
must implement the stream trait `Stream<Item=Bytes, Error=Error>`, i.e:

{{< include-example example="async-handlers" file="stream.rs" section="main" >}}

Both methods can be combined. (i.e Async response with streaming body)

It is possible to return a `Result` where the `Result::Item` type can be `Future`.
In this example, the `index` handler can return an error immediately or return a
future that resolves to a `HttpResponse`.

{{< include-example example="async-handlers" file="async_stream.rs" section="main" >}}

## Different return types (Either)

Sometimes, you need to return different types of responses. For example,
you can error check and return errors, return async responses, or any result that requires two different types.

For this case, the [*Either*](../../actix-web/actix_web/enum.Either.html) type can be used.
`Either` allows combining two different responder types into a single type.

{{< include-example example="either" file="main.rs" section="main" >}}
