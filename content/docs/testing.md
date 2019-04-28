---
title: Testing
menu: docs_advanced
weight: 210
---

# Testing

Every application should be well tested. Actix provides tools to perform unit and
integration tests.

# Unit Tests

For unit testing, actix provides a request builder type and a simple handler runner.
[*TestRequest*](../../actix-web/actix_web/test/struct.TestRequest.html)
implements a builder-like pattern.
You can generate a `HttpRequest` instance with `finish()`, or you can
run your handler with `run()` or `run_async()`.

```rust
use actix_web::{http, test, HttpRequest, HttpResponse, HttpMessage};

fn index(req: &HttpRequest) -> HttpResponse {
     if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(s) = hdr.to_str() {
            return HttpResponse::Ok().into()
        }
     }
     HttpResponse::BadRequest().into()
}

fn main() {
    let resp = test::TestRequest::with_header("content-type", "text/plain")
        .run(&index)
        .unwrap();
    assert_eq!(resp.status(), http::StatusCode::OK);

    let resp = test::TestRequest::default()
        .run(&index)
        .unwrap();
    assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
}
```

# Integration tests

For integration testing, Actix provides a way to init the test app via [*init_service*](../../actix-web/actix_web/test/fn.init_service.html).

Here's an example how to use the method with a test app configuration:

```rust
use actix_service::Service;
use actix_web::{test, web, App, HttpResponse, http::StatusCode};

fn main() {
    let mut app = test::init_service(
        App::new()
            .service(web::resource("/test").to(|| HttpResponse::Ok()))
    );

    // Create request object
    let req = test::TestRequest::with_uri("/test").to_request();

    // Execute application
    let resp = test::block_on(app.call(req)).unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}
```

In a real life, what's usually needed is to test an existing application configuration and, preferrably have it defined only once while it can be reused
both in tests and in production.

Actix provides a way to do this with the [configure](../../actix-web/actix_web/struct.App.html#method.configure) method of an `App` struct.

First, define the method that will be configuring an `App` in the place that can be exported later in both tests and the production code:

```rust
// file src/lib.rs
use actix_web::{HttpResponse, web};

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/test").to(|| HttpResponse::Ok()
        .content_type("test/plain").body("This is a test response")));
}
```

Then use this method in the production code:

```rust
// file src/main.rs
use actix_web::{App, HttpServer};

use my_test_app::config_app;

fn main() -> std::io::Result<()> {
    let sys = actix_rt::System::new("my_test_app");
    HttpServer::new(|| App::new().configure(config_app))
        .bind("127.0.0.1:8888")?
        .start();

    println!("Starting http server: 127.0.0.1:8888");
    sys.run()
}
```

and the testing code:

```rust
// file tests/integration-tests.rs

use actix_service::Service;
use actix_web::{App, http::StatusCode, test};
use actix_web::dev::{Body, ResponseBody, ServiceResponse};

use my_test_app::config_app;

#[test]
fn integration_test() {
    let mut app = test::init_service(App::new().configure(config_app));
    let request = test::TestRequest::with_uri("/test").to_request();

    let response = test::block_on(app.call(request)).unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let response_body = String::from_utf8(test::read_body(response).to_vec())
        .expect("Failed to read response bytes into string");
    assert_eq!(response_body, "This is a test response");
}
```

# Stream response tests

If you need to test stream it would be enough to convert a [*ClientResponse*](../../actix-web/actix_web/client/struct.ClientResponse.html) to future and execute it.
For example of testing [*Server Sent Events*](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events).

```rust
extern crate bytes;
extern crate futures;
extern crate actix_web;

use bytes::Bytes;
use futures::stream::poll_fn;
use futures::{Async, Poll, Stream};

use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse};
use actix_web::http::{ContentEncoding, StatusCode};
use actix_web::test::TestServer;


fn sse(_req: &HttpRequest) -> HttpResponse {
    let mut counter = 5usize;
    // yields `data: N` where N in [5; 1]
    let server_events = poll_fn(move || -> Poll<Option<Bytes>, Error> {
        if counter == 0 {
            return Ok(Async::Ready(None));
        }
        let payload = format!("data: {}\n\n", counter);
        counter -= 1;
        Ok(Async::Ready(Some(Bytes::from(payload))))
    });

    HttpResponse::build(StatusCode::OK)
        .content_encoding(ContentEncoding::Identity)
        .content_type("text/event-stream")
        .streaming(server_events)
}


fn main() {
    // start new test server
    let mut srv = TestServer::new(|app| app.handler(sse));

    // request stream
    let request = srv.get().finish().unwrap();
    let response = srv.execute(request.send()).unwrap();
    assert!(response.status().is_success());

    // convert ClientResponse to future, start read body and wait first chunk
    let mut stream = response.payload();
    loop {
        match srv.execute(stream.into_future()) {
            Ok((Some(bytes), remain)) => {
                println!("{:?}", bytes);
                stream = remain
            }
            Ok((None, _)) => break,
            Err(_) => panic!(),
        }
    }
}
```

# WebSocket server tests

It is possible to register a *handler* with `TestApp::handler()`, which
initiates a web socket connection. *TestServer* provides the method `ws()`, which connects to
the websocket server and returns ws reader and writer objects. *TestServer* also
provides an `execute()` method, which runs future objects to completion and returns
result of the future computation.

The following example demonstrates how to test a websocket handler:

```rust
use actix::{Actor, StreamHandler};
use actix_web::*;
use futures::Stream;

struct Ws;   // <- WebSocket actor

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for Ws {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Text(text) => ctx.text(text),
            _ => (),
        }
    }
}

fn main() {
    let mut srv = test::TestServer::new(
        |app| app.handler(|req| ws::start(req, Ws)));

    let (reader, mut writer) = srv.ws().unwrap();
    writer.text("text");

    let (item, reader) = srv.execute(reader.into_future()).unwrap();
    assert_eq!(item, Some(ws::Message::Text("text".to_owned())));
}
```
