---
title: Extractors
menu: docs_basics
weight: 170
---

# Type-safe information extraction

Actix Web provides a facility for type-safe request information access called _extractors_ (i.e., `impl FromRequest`). There are lots of built-in extractor implementations (see [implementors](https://actix.rs/actix-web/actix_web/trait.FromRequest.html#implementors)).

An extractor can be accessed as an argument to a handler function. Actix Web supports up to 12 extractors per handler function. Argument position does not matter.

{{< include-example example="extractors" file="main.rs" section="option-one" >}}

# Path

[_Path_][pathstruct] provides information that is extracted from the request's path. Parts of the path that are extractable are called "dynamic segments" and are marked with curly braces. You can deserialize any variable segment from the path.

For instance, for resource that registered for the `/users/{user_id}/{friend}` path, two segments could be deserialized, `user_id` and `friend`. These segments could be extracted as a tuple in the order they are declared (e.g., `Path<(u32, String)>`).

{{< include-example example="extractors" file="path_one.rs" section="path-one" >}}

It is also possible to extract path information to a type that implements the `Deserialize` trait from `serde` by matching dynamic segment names with field names. Here is an equivalent example that uses `serde` instead of a tuple type.

{{< include-example example="extractors" file="path_two.rs" section="path-two" >}}

As a non-type-safe alternative, it's also possible to `query` the request for path parameters by name within a handler:

{{< include-example example="extractors" file="path_three.rs" section="path-three" >}}

# Query

The [_Query_][querystruct] type provides extraction functionality for the request's query parameters. Underneath it uses _serde_urlencoded_ crate.

{{< include-example example="extractors" file="query.rs" section="query" >}}

# Json

[_Json_][jsonstruct] allows deserialization of a request body into a struct. To extract typed information from a request's body, the type `T` must implement the `Deserialize` trait from _serde_.

{{< include-example example="extractors" file="json_one.rs" section="json-one" >}}

Some extractors provide a way to configure the extraction process. To configure an extractor, pass its configuration object to the resource's `.app_data()` method. In the case of _Json_ extractor it returns a [_JsonConfig_][jsonconfig]. You can configure the maximum size of the JSON payload as well as a custom error handler function.

The following example limits the size of the payload to 4kb and uses a custom error handler.

{{< include-example example="extractors" file="json_two.rs" section="json-two" >}}

# Form

At the moment, only url-encoded forms are supported. The url-encoded body could be extracted to a specific type. This type must implement the `Deserialize` trait from the _serde_ crate.

[_FormConfig_][formconfig] allows configuring the extraction process.

{{< include-example example="extractors" file="form.rs" section="form" >}}

# Other

Actix Web also provides several other extractors:

- [_Data_][datastruct] - If you need access to an application state.
- _HttpRequest_ - _HttpRequest_ itself is an extractor which returns self, in case you need access to the request.
- _String_ - You can convert a request's payload to a _String_. [_Example_][stringexample] is available in doc strings.
- _actix_web::web::Bytes_ - You can convert a request's payload into _Bytes_. [_Example_][bytesexample] is available in doc strings.
- _Payload_ - You can access a request's payload. [_Example_][payloadexample]

# Application state extractor

Application state is accessible from the handler with the `web::Data` extractor; however, state is accessible as a read-only reference. If you need mutable access to state, it must be implemented.

> **Beware**, actix creates multiple copies of the application state and the handlers. It creates one copy for each thread.

Here is an example of a handler that stores the number of processed requests:

{{< include-example example="request-handlers" file="main.rs" section="data" >}}

Although this handler will work, `data.count` will only count the number of requests handled _by each thread_. To count the number of total requests across all threads, one should use `Arc` and [atomics][atomics].

{{< include-example example="request-handlers" file="handlers_arc.rs" section="arc" >}}

> **Note**, if you want the _entire_ state to be shared across all threads, use `web::Data` and `app_data` as described in [Shared Mutable State][shared_mutable_state].

> Be careful with synchronization primitives like `Mutex` or `RwLock`. The `actix-web` framework handles requests asynchronously. By blocking thread execution, all concurrent request handling processes would block. If you need to share or update some state from multiple threads, consider using the tokio synchronization primitives.

[pathstruct]: https://docs.rs/actix-web/4/actix_web/dev/struct.Path.html
[querystruct]: https://docs.rs/actix-web/4/actix_web/web/struct.Query.html
[jsonstruct]: https://docs.rs/actix-web/4/actix_web/web/struct.Json.html
[jsonconfig]: https://docs.rs/actix-web/4/actix_web/web/struct.JsonConfig.html
[formconfig]: https://docs.rs/actix-web/4/actix_web/web/struct.FormConfig.html
[datastruct]: https://docs.rs/actix-web/4/actix_web/web/struct.Data.html
[stringexample]: https://docs.rs/actix-web/4/actix_web/trait.FromRequest.html#example-2
[bytesexample]: https://docs.rs/actix-web/4/actix_web/trait.FromRequest.html#example-4
[payloadexample]: https://docs.rs/actix-web/4/actix_web/web/struct.Payload.html
[actix]: https://actix.github.io/actix/actix/
[atomics]: https://doc.rust-lang.org/std/sync/atomic/
[shared_mutable_state]: ../application#shared-mutable-state
