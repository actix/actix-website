mod async_stream;
mod stream;
// <main>
use actix_web::{App, AsyncResponder, Error, HttpRequest, HttpResponse};
use futures::future::{result, Future};

fn index(_req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    result(Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("Hello!"))))
    .responder()
}

fn index2(_req: &HttpRequest) -> Box<Future<Item = &'static str, Error = Error>> {
    result(Ok("Welcome!")).responder()
}

fn main() {
    App::new()
        .resource("/async", |r| r.route().a(index))
        .resource("/", |r| r.route().a(index2))
        // .resource("/", |r| r.route().f(async_stream::index))
        .finish();
}
// </main>
