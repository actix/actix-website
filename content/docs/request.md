---
title: Requests
menu: docs_advanced
weight: 200
---

# Content Encoding

Actix automatically *decompresses* payloads. The following codecs are supported:

* Brotli
* Gzip
* Deflate
* Identity

If request headers contain a `Content-Encoding` header, the request payload is decompressed
according to the header value. Multiple codecs are not supported,
i.e: `Content-Encoding: br, gzip`.

# JSON Request

There are several options for json body deserialization.

The first option is to use *Json* extractor. First, you define a handler function
that accepts `Json<T>` as a parameter, then, you use the `.with()` method for registering
this handler. It is also possible to accept arbitrary valid json object by
using `serde_json::Value` as a type `T`.

```rust
#[macro_use] extern crate serde_derive;
use actix_web::{App, Json, Result, http};

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// extract `Info` using serde
fn index(info: Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

fn main() {
    let app = App::new().resource(
       "/index.html",
       |r| r.method(http::Method::POST).with(index));  // <- use `with` extractor
}
```

Another option is to use *HttpRequest::json()*. This method returns a
[*JsonBody*](../../actix-web/actix_web/dev/struct.JsonBody.html) object which resolves into
the deserialized value.

```rust
#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

fn index(req: &HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    req.json().from_err()
        .and_then(|val: MyObj| {
            println!("model: {:?}", val);
            Ok(HttpResponse::Ok().json(val))  // <- send response
        })
        .responder()
}
```

You may also manually load the payload into memory and then deserialize it.

In the following example, we will deserialize a *MyObj* struct. We need to load the request
body first and then deserialize the json into an object.

```rust
extern crate serde_json;
use futures::{Future, Stream};

#[derive(Serialize, Deserialize)]
struct MyObj {name: String, number: i32}

fn index(req: &HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
   req.payload()
      // `concat2` will asynchronously read each chunk of the request body and
      // return a single, concatenated, chunk
      .concat2()
      // `Future::from_err` acts like `?` in that it coerces the error type from
      // the future into the final error type
      .from_err()
      // `Future::and_then` can be used to merge an asynchronous workflow with a
      // synchronous workflow
      .and_then(|body| {
          let obj = serde_json::from_slice::<MyObj>(&body)?;
          Ok(HttpResponse::Ok().json(obj))
      })
      .responder()
}
```

> A complete example for both options is available in
> [examples directory](https://github.com/actix/examples/tree/master/json/).

# Chunked transfer encoding

Actix automatically decodes *chunked* encoding. `HttpRequest::payload()` already contains
the decoded byte stream. If the request payload is compressed with one of the supported
compression codecs (br, gzip, deflate), then the byte stream is decompressed.

# Multipart body

Actix provides multipart stream support.
[*Multipart*](../../actix-web/actix_web/multipart/struct.Multipart.html) is implemented as
a stream of multipart items. Each item can be a
[*Field*](../../actix-web/actix_web/multipart/struct.Field.html) or a nested
*Multipart* stream.`HttpResponse::multipart()` returns the *Multipart* stream
for the current request.

The following demonstrates multipart stream handling for a simple form:

```rust
use actix_web::*;

fn index(req: &HttpRequest) -> Box<Future<...>> {
    // get multipart and iterate over multipart items
    req.multipart()
       .and_then(|item| {
           match item {
              multipart::MultipartItem::Field(field) => {
                 println!("==== FIELD ==== {:?} {:?}",
                          field.headers(),
                          field.content_type());
                 Either::A(
                   field.map(|chunk| {
                        println!("-- CHUNK: \n{}",
                                 std::str::from_utf8(&chunk).unwrap());})
                      .fold((), |_, _| result(Ok(()))))
                },
              multipart::MultipartItem::Nested(mp) => {
                 Either::B(result(Ok(())))
              }
         }
   })
}
```

> A full example is available in the
> [examples directory](https://github.com/actix/examples/tree/master/multipart/).

# Urlencoded body

Actix provides support for *application/x-www-form-urlencoded* encoded bodies.
`HttpResponse::urlencoded()` returns a
[*UrlEncoded*](../../actix-web/actix_web/dev/struct.UrlEncoded.html) future, which resolves
to the deserialized instance. The type of the instance must implement the
`Deserialize` trait from *serde*.

The *UrlEncoded* future can resolve into an error in several cases:

* content type is not `application/x-www-form-urlencoded`
* transfer encoding is `chunked`.
* content-length is greater than 256k
* payload terminates with error.

```rust
#[macro_use] extern crate serde_derive;
use actix_web::*;
use futures::future::{Future, ok};

#[derive(Deserialize)]
struct FormData {
    username: String,
}

fn index(req: &HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    req.urlencoded::<FormData>() // <- get UrlEncoded future
       .from_err()
       .and_then(|data| {        // <- deserialized instance
             println!("USERNAME: {:?}", data.username);
             ok(HttpResponse::Ok().into())
       })
       .responder()
}
# fn main() {}
```

# Streaming request

*HttpRequest* is a stream of `Bytes` objects. It can be used to read the request
body payload.

In the following example, we read and print the request payload chunk by chunk:

```rust
use actix_web::*;
use futures::{Future, Stream};


fn index(req: &HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    req
       .payload()
       .from_err()
       .fold((), |_, chunk| {
            println!("Chunk: {:?}", chunk);
            result::<_, error::PayloadError>(Ok(()))
        })
       .map(|_| HttpResponse::Ok().finish())
       .responder()
}
```
