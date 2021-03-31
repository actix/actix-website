---
title: Extractors
menu: docs_basics
weight: 170
---

# Type-safe information extraction

Actix-web provides a facility for type-safe request information access called *extractors*
(i.e., `impl FromRequest`). By default, actix-web provides several extractor implementations.

An extractor can be accessed as an argument to a handler function. Actix-web supports
up to 12 extractors per handler function. Argument position does not matter.

{{< include-example example="extractors" file="main.rs" section="option-one" >}}

# Path

[*Path*][pathstruct] provides information that can be extracted from the Request's
path. You can deserialize any variable segment from the path.

For instance, for resource that registered for the `/users/{user_id}/{friend}` path,
two segments could be deserialized, `user_id` and `friend`. These segments could be
extracted into a `tuple`, i.e. `Path<(u32, String)>` or any structure that implements
the `Deserialize` trait from the *serde* crate.

{{< include-example example="extractors" file="path_one.rs" section="path-one" >}}

It is also possible to extract path information to a specific type that implements the
`Deserialize` trait from *serde*. Here is an equivalent example that uses *serde*
instead of a *tuple* type.

{{< include-example example="extractors" file="path_two.rs" section="path-two" >}}

It is also possible to `get` or `query` the request for path parameters by name:

{{< include-example example="extractors" file="path_three.rs" section="path-three" >}}

# Query

The [*Query*][querystruct] type provides extraction functionality for the request's
query parameters. Underneath it uses *serde_urlencoded* crate.

{{< include-example example="extractors" file="query.rs" section="query" >}}

# Json

[*Json*][jsonstruct] allows deserialization of a request body into a struct. To extract
typed information from a request's body, the type `T` must implement the `Deserialize`
trait from *serde*.

{{< include-example example="extractors" file="json_one.rs" section="json-one" >}}

Some extractors provide a way to configure the extraction process. To configure
an extractor, pass its configuration object to the resource's `.data()` method.
In the case of *Json* extractor it returns a [*JsonConfig*][jsonconfig].
You can configure the maximum size of the JSON payload as
well as a custom error handler function.

The following example limits the size of the payload to 4kb and uses a custom error handler.

{{< include-example example="extractors" file="json_two.rs" section="json-two" >}}

# Form

At the moment, only url-encoded forms are supported. The url-encoded body could be
extracted to a specific type. This type must implement the `Deserialize` trait from
the *serde* crate.

[*FormConfig*][formconfig] allows configuring the extraction process.

{{< include-example example="extractors" file="form.rs" section="form" >}}

# Other

Actix-web also provides several other extractors:

* [*Data*][datastruct] - If you need access to an application state.
* *HttpRequest* - *HttpRequest* itself is an extractor which returns self, in case you
  need access to the request.
* *String* - You can convert a request's payload to a *String*.  [*Example*][stringexample]
  is available in doc strings.
* *actix_web::web::Bytes* - You can convert a request's payload into *Bytes*.
  [*Example*][bytesexample]
  is available in doc strings.
* *Payload* - You can access a request's payload.
  [*Example*][payloadexample]

# Application state extractor

Application state is accessible from the handler with the `web::Data` extractor;
however, state is accessible as a read-only reference. If you need mutable access to state,
it must be implemented.

> **Beware**, actix creates multiple copies of the application state and the handlers. It creates
> one copy for each thread.

Here is an example of a handler that stores the number of processed requests:

{{< include-example example="request-handlers" file="main.rs" section="data" >}}

Although this handler will work, `self.0` will be different depending on the number of threads and
number of requests processed per thread. A proper implementation would use `web::Data` and `AtomicUsize`.

{{< include-example example="request-handlers" file="handlers_arc.rs" section="arc" >}}

> Be careful with synchronization primitives like `Mutex` or `RwLock`. The `actix-web` framework
> handles requests asynchronously. By blocking thread execution, all concurrent
> request handling processes would block. If you need to share or update some state
> from multiple threads, consider using the tokio synchronization primitives.

[pathstruct]: https://docs.rs/actix-web/3/actix_web/dev/struct.Path.html
[querystruct]: https://docs.rs/actix-web/3/actix_web/web/struct.Query.html
[jsonstruct]: https://docs.rs/actix-web/3/actix_web/web/struct.Json.html
[jsonconfig]: https://docs.rs/actix-web/3/actix_web/web/struct.JsonConfig.html
[formconfig]: https://docs.rs/actix-web/3/actix_web/web/struct.FormConfig.html
[datastruct]: https://docs.rs/actix-web/3/actix_web/web/struct.Data.html
[stringexample]: https://docs.rs/actix-web/3/actix_web/trait.FromRequest.html#example-2
[bytesexample]: https://docs.rs/actix-web/3/actix_web/trait.FromRequest.html#example-4
[payloadexample]: https://docs.rs/actix-web/3/actix_web/web/struct.Payload.html
[actix]: https://actix.github.io/actix/actix/
