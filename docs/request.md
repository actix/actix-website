---
title: Requests
---

import CodeBlock from "@site/src/components/code_block";

# JSON Request

There are several options for json body deserialization.

The first option is to use _Json_ extractor. First, you define a handler function that accepts `Json<T>` as a parameter, then, you use the `.to()` method for registering this handler. It is also possible to accept arbitrary valid json object by using `serde_json::Value` as a type `T`.

First example of json of `JSON Request` depends on `serde`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

Second example of `JSON Request` depends on `serde` and `serde_json` and `futures`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1"
futures = "0.3"
```

If you want to add default value for a field, refer to `serde`'s [documentation](https://serde.rs/attr-default.html).

<CodeBlock example="requests" file="main.rs" section="json-request" />

You may also manually load the payload into memory and then deserialize it.

In the following example, we will deserialize a _MyObj_ struct. We need to load the request body first and then deserialize the json into an object.

<CodeBlock example="requests" file="manual.rs" section="json-manual" />

> A complete example for both options is available in [examples directory][examples].

## Content Encoding

Actix Web automatically _decompresses_ payloads. The following codecs are supported:

- Brotli
- Gzip
- Deflate
- Zstd

If request headers contain a `Content-Encoding` header, the request payload is decompressed according to the header value. Multiple codecs are not supported, i.e: `Content-Encoding: br, gzip`.

## Chunked transfer encoding

Actix automatically decodes _chunked_ encoding. The [`web::Payload`][payloadextractor] extractor already contains the decoded byte stream. If the request payload is compressed with one of the supported compression codecs (br, gzip, deflate), then the byte stream is decompressed.

## Multipart body

Actix Web provides multipart stream support with an external crate, [`actix-multipart`][multipartcrate].

> A full example is available in the [examples directory][multipartexample].

## Urlencoded body

Actix Web provides support for _application/x-www-form-urlencoded_ encoded bodies with the [`web::Form`][formencoded] extractor which resolves to the deserialized instance. The type of the instance must implement the `Deserialize` trait from _serde_.

The _UrlEncoded_ future can resolve into an error in several cases:

- content type is not `application/x-www-form-urlencoded`
- transfer encoding is `chunked`.
- content-length is greater than 256k
- payload terminates with error.

<CodeBlock example="requests" file="urlencoded.rs" section="urlencoded" />

## Streaming request

_HttpRequest_ is a stream of `Bytes` objects. It can be used to read the request body payload.

In the following example, we read and print the request payload chunk by chunk:

<CodeBlock example="requests" file="streaming.rs" section="streaming" />

[examples]: https://github.com/actix/examples/tree/master/json/json
[multipartstruct]: https://docs.rs/actix-multipart/0.2/actix_multipart/struct.Multipart.html
[fieldstruct]: https://docs.rs/actix-multipart/0.2/actix_multipart/struct.Field.html
[multipartexample]: https://github.com/actix/examples/tree/master/forms/multipart
[urlencoded]: https://docs.rs/actix-web/4/actix_web/dev/struct.UrlEncoded.html
[payloadextractor]: https://docs.rs/actix-web/4/actix_web/web/struct.Payload.html
[multipartcrate]: https://crates.io/crates/actix-multipart
[formencoded]: https://docs.rs/actix-web/4/actix_web/web/struct.Form.html
