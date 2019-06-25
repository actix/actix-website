---
title: Requests
menu: docs_advanced
weight: 200
---

# Content Encoding

Actix automatically *decompresses* payloads. The following codecs are supported:

* Brotli
* Chunked
* Compress
* Gzip
* Deflate
* Identity
* Trailers
* EncodingExt

If request headers contain a `Content-Encoding` header, the request payload is decompressed
according to the header value. Multiple codecs are not supported, i.e: `Content-Encoding: br, gzip`.

# JSON Request

There are several options for json body deserialization.

The first option is to use *Json* extractor. First, you define a handler function
that accepts `Json<T>` as a parameter, then, you use the `.to()` method for registering
this handler. It is also possible to accept arbitrary valid json object by
using `serde_json::Value` as a type `T`.

{{< include-example example="requests" file="main.rs" section="json-request" >}}

You may also manually load the payload into memory and then deserialize it.

In the following example, we will deserialize a *MyObj* struct. We need to load the request
body first and then deserialize the json into an object.

{{< include-example example="requests" file="manual.rs" section="json-manual" >}}

> A complete example for both options is available in [examples directory][examples].

# Chunked transfer encoding

Actix automatically decodes *chunked* encoding. `HttpRequest::payload()` already contains
the decoded byte stream. If the request payload is compressed with one of the supported
compression codecs (br, gzip, deflate), then the byte stream is decompressed.

# Multipart body

Actix provides multipart stream support.
[*Multipart*][multipartstruct] is implemented as a stream of multipart items. Each item
can be a [*Field*][fieldstruct] or a nested *Multipart* stream.`HttpResponse::multipart()`
returns the *Multipart* stream for the current request.

The following demonstrates multipart stream handling for a simple form:

{{< include-example example="requests" file="multipart.rs" section="multipart" >}}

> A full example is available in the [examples directory][multipartexample].

# Urlencoded body

Actix provides support for *application/x-www-form-urlencoded* encoded bodies.
`HttpResponse::urlencoded()` returns a [*UrlEncoded*][urlencoder] future, which resolves
to the deserialized instance. The type of the instance must implement the `Deserialize`
trait from *serde*.

The *UrlEncoded* future can resolve into an error in several cases:

* content type is not `application/x-www-form-urlencoded`
* transfer encoding is `chunked`.
* content-length is greater than 256k
* payload terminates with error.

{{< include-example example="requests" file="urlencoded.rs" section="urlencoded" >}}

# Streaming request

*HttpRequest* is a stream of `Bytes` objects. It can be used to read the request
body payload.

In the following example, we read and print the request payload chunk by chunk:

{{< include-example example="requests" file="streaming.rs" section="streaming" >}}

[examples]: https://github.com/actix/examples/tree/master/json/
[multipartstruct]: ../../actix-web/actix_web/multipart/struct.Multipart.html
[fieldstruct]: ../../actix-web/actix_web/multipart/struct.Field.html
[multipartexample]: https://github.com/actix/examples/tree/master/multipart/
[urlencoder]: ../../actix-web/actix_web/dev/struct.UrlEncoded.html
