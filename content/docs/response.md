---
title: Responses
menu: docs_advanced
weight: 210
---

# Response

A builder-like pattern is used to construct an instance of `HttpResponse`.
`HttpResponse` provides several methods that return a `HttpResponseBuilder` instance,
which implements various convenience methods for building responses.

> Check the [documentation](../../actix-web/actix_web/dev/struct.HttpResponseBuilder.html)
> for type descriptions.

The methods `.body`, `.finish`, and `.json` finalize response creation and
return a constructed *HttpResponse* instance. If this methods is called on the same
builder instance multiple times, the builder will panic.

```rust
use actix_web::{HttpRequest, HttpResponse, http::ContentEncoding};

fn index(req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_encoding(ContentEncoding::Br)
        .content_type("plain/text")
        .header("X-Hdr", "sample")
        .body("data")
}
```

# Content encoding

Actix automatically *compresses* payloads. The following codecs are supported:

* Brotli
* Gzip
* Deflate
* Identity

Response payload is compressed based on the *content_encoding* parameter.
By default, `ContentEncoding::Auto` is used. If `ContentEncoding::Auto` is selected,
then the compression depends on the request's `Accept-Encoding` header.

> `ContentEncoding::Identity` can be used to disable compression.
> If another content encoding is selected, the compression is enforced for that codec.

For example, to enable `brotli` use `ContentEncoding::Br`:

```rust
use actix_web::{HttpRequest, HttpResponse, http::ContentEncoding};

fn index(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_encoding(ContentEncoding::Br)
        .body("data")
}
```

In this case we explicitly disable content compression
by setting content encoding to a `Identity` value:

```rust
use actix_web::{HttpRequest, HttpResponse, http::ContentEncoding};

fn index(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        // v- disable compression
        .content_encoding(ContentEncoding::Identity)
        .body("data")
}
```

When dealing with an already compressed body (for example when serving assets),
set the content encoding to `Identity` to avoid compressing the already compressed 
data and set the `content-encoding` header manually:

```rust
use actix_web::{HttpRequest, HttpResponse, http::ContentEncoding};

static HELLO_WORLD: &[u8] = &[
    0x1f,0x8b,0x08,0x00,0xa2,0x30,0x10,0x5c,
    0x00,0x03,0xcb,0x48,0xcd,0xc9,0xc9,0x57,
    0x28,0xcf,0x2f,0xca,0x49,0xe1,0x02,0x00,
    0x2d,0x3b,0x08,0xaf,0x0c,0x00,0x00,0x00
];

pub fn index(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_encoding(ContentEncoding::Identity)
        .header("content-encoding", "gzip")
        .body(HELLO_WORLD)
}
```

Also it is possible to set default content encoding on application level, by
default `ContentEncoding::Auto` is used, which implies automatic content compression
negotiation.

```rust
use actix_web::{App, HttpRequest, HttpResponse, http::ContentEncoding};

fn index(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .body("data")
}

fn main() {
    let app = App::new()
        // v- disable compression for all routes
       .default_encoding(ContentEncoding::Identity)
       .resource("/index.html", |r| r.with(index));
}
```

# JSON Response

The `Json` type allows to respond with well-formed JSON data: simply return a value of
type Json<T> where `T` is the type of a structure to serialize into *JSON*.
The type `T` must implement the `Serialize` trait from *serde*.

```rust
# extern crate actix_web;
#[macro_use] extern crate serde_derive;
use actix_web::{App, HttpRequest, Json, Result, http::Method};

#[derive(Serialize)]
struct MyObj {
    name: String,
}

fn index(req: &HttpRequest) -> Result<Json<MyObj>> {
    Ok(Json(MyObj{name: req.match_info().query("name")?}))
}

fn main() {
    App::new()
        .resource(r"/a/{name}", |r| r.method(Method::GET).f(index))
        .finish();
}
```

# Chunked transfer encoding

Chunked encoding on a response can be enabled with `HttpResponseBuilder::chunked()`.
This takes effect only for `Body::Streaming(BodyStream)` or `Body::StreamingContext` bodies.
If the response payload compression is enabled and a streaming body is used, chunked encoding
is enabled automatically.

> Enabling chunked encoding for *HTTP/2.0* responses is forbidden.

```rust
use actix_web::*;
use bytes::Bytes;
use futures::stream::once;

fn index(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .chunked()
        .body(Body::Streaming(Box::new(once(Ok(Bytes::from_static(b"data"))))))
}
```
