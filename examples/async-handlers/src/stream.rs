// <main>
use actix_web::{App, Body, HttpRequest, HttpResponse};
use bytes::Bytes;
use futures::stream::once;

fn index(_req: &HttpRequest) -> HttpResponse {
    let body = once(Ok(Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .body(Body::Streaming(Box::new(body)))
}

pub fn main() {
    App::new().resource("/async", |r| r.f(index)).finish();
}
// </main>
