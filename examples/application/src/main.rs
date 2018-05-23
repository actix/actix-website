extern crate actix_web;
use actix_web::{App, Responder, HttpRequest, http::Method};

// <setup>
fn index(req: HttpRequest) -> impl Responder {
    "Hello world!"
}

fn main() {
    let app = App::new()
        .prefix("/app")
        .resource("/index.html", |r| r.method(Method::GET).f(index))
        .finish();
}
// </setup>
