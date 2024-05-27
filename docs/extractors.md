---
title: Extractors
---

import CodeBlock from "@site/src/components/code_block";

# Type-safe information extraction

Actix Web provides a facility for type-safe request information access called _extractors_ (i.e., `impl FromRequest`). There are lots of built-in extractor implementations (see [implementors](https://docs.rs/actix-web/latest/actix_web/trait.FromRequest.html#implementors)).

An extractor can be accessed as an argument to a handler function. Actix Web supports up to 12 extractors per handler function. Argument position does not matter.

<CodeBlock example="extractors" file="main.rs" section="option-one" />

## Path

[_Path_][pathstruct] provides information that is extracted from the request's path. Parts of the path that are extractable are called "dynamic segments" and are marked with curly braces. You can deserialize any variable segment from the path.

For instance, for resource that registered for the `/users/{user_id}/{friend}` path, two segments could be deserialized, `user_id` and `friend`. These segments could be extracted as a tuple in the order they are declared (e.g., `Path<(u32, String)>`).

<CodeBlock example="extractors" file="path_one.rs" section="path-one" />

It is also possible to extract path information to a type that implements the `Deserialize` trait from `serde` by matching dynamic segment names with field names. Here is an equivalent example that uses a deserialization struct using `serde` (make sure to enable its `derive` feature) instead of a tuple type.

<CodeBlock example="extractors" file="path_two.rs" section="path-two" />

As a non-type-safe alternative, it's also possible to query (see [`match_info` docs][docsrs_match_info]) the request for path parameters by name within a handler:

<CodeBlock example="extractors" file="path_three.rs" section="path-three" />

## Query

The [`Query<T>`][querystruct] type provides extraction functionality for the request's query parameters. Underneath it uses `serde_urlencoded` crate.

<CodeBlock example="extractors" file="query.rs" section="query" />

## JSON

[`Json<T>`][jsonstruct] allows deserialization of a request body into a struct. To extract typed information from a request's body, the type `T` must implement `serde::Deserialize`.

<CodeBlock example="extractors" file="json_one.rs" section="json-one" />

Some extractors provide a way to configure the extraction process. To configure an extractor, pass its configuration object to the resource's `.app_data()` method. In the case of _Json_ extractor it returns a [_JsonConfig_][jsonconfig]. You can configure the maximum size of the JSON payload as well as a custom error handler function.

The following example limits the size of the payload to 4kb and uses a custom error handler.

<CodeBlock example="extractors" file="json_two.rs" section="json-two" />

## URL-Encoded Forms

A URL-encoded form body can be extracted to a struct, much like `Json<T>`. This type must implement `serde::Deserialize`.

[_FormConfig_][formconfig] allows configuring the extraction process.

<CodeBlock example="extractors" file="form.rs" section="form" />

## Other

Actix Web also provides many other extractors, here's a few important ones:

- [`Data`][datastruct] - For accessing pieces of application state.
- [`HttpRequest`][httprequest] - `HttpRequest` is itself an extractor, in case you need access to other parts of the request.
- `String` - You can convert a request's payload to a `String`. [_An example_][stringexample] is available in the rustdoc.
- [`Bytes`][bytes] - You can convert a request's payload into _Bytes_. [_An example_][bytesexample] is available in the rustdoc.
- [`Payload`][payload] - Low-level payload extractor primarily for building other extractors. [_An example_][payloadexample] is available in the rustdoc.

## Application State Extractor

Application state is accessible from the handler with the `web::Data` extractor; however, state is accessible as a read-only reference. If you need mutable access to state, it must be implemented.

Here is an example of a handler that stores the number of processed requests:

<CodeBlock example="request-handlers" file="main.rs" section="data" />

Although this handler will work, `data.count` will only count the number of requests handled _by each worker thread_. To count the number of total requests across all threads, one should use shared `Arc` and [atomics][atomics].

<CodeBlock example="request-handlers" file="handlers_arc.rs" section="arc" />

**Note**: If you want the _entire_ state to be shared across all threads, use `web::Data` and `app_data` as described in [Shared Mutable State][shared_mutable_state].

Be careful when using blocking synchronization primitives like `Mutex` or `RwLock` within your app state. Actix Web handles requests asynchronously. It is a problem if the [_critical section_][critical_section] in your handler is too big or contains an `.await` point. If this is a concern, we would advise you to also read [Tokio's advice on using blocking `Mutex` in async code][tokio_std_mutex].

[pathstruct]: https://docs.rs/actix-web/4/actix_web/dev/struct.Path.html
[querystruct]: https://docs.rs/actix-web/4/actix_web/web/struct.Query.html
[jsonstruct]: https://docs.rs/actix-web/4/actix_web/web/struct.Json.html
[jsonconfig]: https://docs.rs/actix-web/4/actix_web/web/struct.JsonConfig.html
[formconfig]: https://docs.rs/actix-web/4/actix_web/web/struct.FormConfig.html
[datastruct]: https://docs.rs/actix-web/4/actix_web/web/struct.Data.html
[httprequest]: https://docs.rs/actix-web/4/actix_web/struct.HttpRequest.html
[stringexample]: https://docs.rs/actix-web/4/actix_web/trait.FromRequest.html#impl-FromRequest-for-String
[bytes]: https://docs.rs/actix-web/4/actix_web/web/struct.Bytes.html
[bytesexample]: https://docs.rs/actix-web/4/actix_web/trait.FromRequest.html#impl-FromRequest-5
[payload]: https://docs.rs/actix-web/4/actix_web/web/struct.Payload.html
[payloadexample]: https://docs.rs/actix-web/4/actix_web/web/struct.Payload.html
[docsrs_match_info]: https://docs.rs/actix-web/latest/actix_web/struct.HttpRequest.html#method.match_info
[actix]: /actix/docs/
[atomics]: https://doc.rust-lang.org/std/sync/atomic/
[shared_mutable_state]: /docs/application#shared-mutable-state
[critical_section]: https://en.wikipedia.org/wiki/Critical_section
[tokio_std_mutex]: https://tokio.rs/tokio/tutorial/shared-state#on-using-stdsyncmutex
