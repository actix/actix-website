---
title: Responses
menu: docs_advanced
weight: 210
---

# Response

A builder-like pattern is used to construct an instance of `HttpResponse`.  `HttpResponse`
provides several methods that return a `HttpResponseBuilder` instance, which implements
various convenience methods for building responses.

> Check the [documentation][responsebuilder] for type descriptions.

The methods `.body`, `.finish`, and `.json` finalize response creation and return a
constructed *HttpResponse* instance. If this methods is called on the same builder
instance multiple times, the builder will panic.

{{< include-example example="responses" file="main.rs" section="builder" >}}

# Content encoding

Actix-web can automatically *compresses* payloads with the [*Compress middleware*][compressmidddleware].
The following codecs are supported:

* Brotli
* Gzip
* Deflate
* Identity

{{< include-example example="responses" file="compress.rs" section="compress" >}}

Response payload is compressed based on the *encoding* parameter from the
`middleware::BodyEncoding` trait.  By default, `ContentEncoding::Auto` is used. If
`ContentEncoding::Auto` is selected, then the compression depends on the request's
`Accept-Encoding` header.

> `ContentEncoding::Identity` can be used to disable compression.
> If another content encoding is selected, the compression is enforced for that codec.

For example, to enable `brotli` for a single handler use `ContentEncoding::Br`:

{{< include-example example="responses" file="brotli.rs" section="brotli" >}}

or for the entire application:

{{< include-example example="responses" file="brotli_two.rs" section="brotli-two" >}}

In this case we explicitly disable content compression by setting content encoding to
an `Identity` value:

{{< include-example example="responses" file="identity.rs" section="identity" >}}

When dealing with an already compressed body (for example when serving assets),
set the content encoding to `Identity` to avoid compressing the already compressed
data and set the `content-encoding` header manually:

{{< include-example example="responses" file="identity_two.rs" section="identity-two" >}}

Also it is possible to set default content encoding on application level, by
default `ContentEncoding::Auto` is used, which implies automatic content compression
negotiation.

{{< include-example example="responses" file="auto.rs" section="auto" >}}

# JSON Response

The `Json` type allows to respond with well-formed JSON data: simply return a value of
type `Json<T>` where `T` is the type of a structure to serialize into *JSON*.
The type `T` must implement the `Serialize` trait from *serde*.

{{< include-example example="responses" file="json_resp.rs" section="json-resp" >}}

[responsebuilder]: https://docs.rs/actix-web/2/actix_web/dev/struct.HttpResponseBuilder.html
[compressmidddleware]: https://docs.rs/actix-web/2/actix_web/middleware/struct.Compress.html
