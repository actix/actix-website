---
title: Responses
---

import CodeBlock from "@site/src/components/code_block";

# Response

A builder-like pattern is used to construct an instance of `HttpResponse`. `HttpResponse` provides several methods that return a `HttpResponseBuilder` instance, which implements various convenience methods for building responses.

> Check the [documentation][responsebuilder] for type descriptions.

The methods `.body`, `.finish`, and `.json` finalize response creation and return a constructed _HttpResponse_ instance. If this methods is called on the same builder instance multiple times, the builder will panic.

<CodeBlock example="responses" file="main.rs" section="builder" />

## JSON Response

The `Json` type allows to respond with well-formed JSON data: simply return a value of type `Json<T>` where `T` is the type of a structure to serialize into _JSON_. The type `T` must implement the `Serialize` trait from _serde_.

For the following example to work, you need to add `serde` to your dependencies in `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

<CodeBlock example="responses" file="json_resp.rs" section="json-resp" />

Using the `Json` type this way instead of calling the `.json` method on a `HttpResponse` makes it immediately clear that the function returns JSON and not any other type of response.

## Content encoding

Actix Web can automatically _compress_ payloads with the [_Compress middleware_][compressmidddleware]. The following codecs are supported:

- Brotli
- Gzip
- Deflate
- Identity

A response's `Content-Encoding` header defaults to `ContentEncoding::Auto`, which performs automatic content compression negotiation based on the request's `Accept-Encoding` header.

<CodeBlock example="responses" file="auto.rs" section="auto" />

Explicitly disable content compression on a handler by setting `Content-Encoding` to an `Identity` value:

<CodeBlock example="responses" file="identity.rs" section="identity" />

When dealing with an already compressed body (for example, when serving pre-compressed assets), set the `Content-Encoding` header on the response manually to bypass the middleware:

<CodeBlock example="responses" file="identity_two.rs" section="identity-two" />

[responsebuilder]: https://docs.rs/actix-web/4/actix_web/struct.HttpResponseBuilder.html
[compressmidddleware]: https://docs.rs/actix-web/4/actix_web/middleware/struct.Compress.html
