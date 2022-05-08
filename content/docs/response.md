---
title: Responses
menu: docs_advanced
weight: 210
---

# Response

A builder-like pattern is used to construct an instance of `HttpResponse`. `HttpResponse` provides several methods that return a `HttpResponseBuilder` instance, which implements various convenience methods for building responses.

> Check the [documentation][responsebuilder] for type descriptions.

The methods `.body`, `.finish`, and `.json` finalize response creation and return a constructed _HttpResponse_ instance. If this methods is called on the same builder instance multiple times, the builder will panic.

{{< include-example example="responses" file="main.rs" section="builder" >}}

# JSON Response

The `Json` type allows to respond with well-formed JSON data: simply return a value of type `Json<T>` where `T` is the type of a structure to serialize into _JSON_. The type `T` must implement the `Serialize` trait from _serde_.

For the following example to work, you need to add `serde` to your dependencies in `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

{{< include-example example="responses" file="json_resp.rs" section="json-resp" >}}

Using the `Json` type this way instead of calling the `.json` method on a `HttpResponse` makes it immediately clear that the function returns JSON and not any other type of response.

# Content encoding

Actix Web can automatically _compress_ payloads with the [_Compress middleware_][compressmidddleware]. The following codecs are supported:

- Brotli
- Gzip
- Deflate
- Identity

{{< include-example example="responses" file="compress.rs" section="compress" >}}

Response payload is compressed based on the _encoding_ parameter from the `middleware::BodyEncoding` trait. By default, `ContentEncoding::Auto` is used. If `ContentEncoding::Auto` is selected, then the compression depends on the request's `Accept-Encoding` header.

> `ContentEncoding::Identity` can be used to disable compression. If another content encoding is selected, the compression is enforced for that codec.

For example, to enable `brotli` for a single handler use `ContentEncoding::Br`:

{{< include-example example="responses" file="brotli.rs" section="brotli" >}}

or for the entire application:

{{< include-example example="responses" file="brotli_two.rs" section="brotli-two" >}}

In this case we explicitly disable content compression by setting content encoding to an `Identity` value:

{{< include-example example="responses" file="identity.rs" section="identity" >}}

When dealing with an already compressed body (for example when serving assets), set the content encoding to `Identity` to avoid compressing the already compressed data and set the `content-encoding` header manually:

{{< include-example example="responses" file="identity_two.rs" section="identity-two" >}}

Also it is possible to set default content encoding on application level, by default `ContentEncoding::Auto` is used, which implies automatic content compression negotiation.

{{< include-example example="responses" file="auto.rs" section="auto" >}}

[responsebuilder]: https://docs.rs/actix-web/4/actix_web/struct.HttpResponseBuilder.html
[compressmidddleware]: https://docs.rs/actix-web/4/actix_web/middleware/struct.Compress.html
