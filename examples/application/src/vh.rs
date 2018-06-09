#![allow(unused)]
use actix_web::{http::Method, pred, server, App, HttpRequest, HttpResponse, Responder};

// <vh>
fn main() {
    let server = server::new(|| {
        vec![
            App::new()
                .filter(pred::Host("www.rust-lang.org"))
                .resource("/", |r| r.f(|r| HttpResponse::Ok())),
            App::new()
                .filter(pred::Host("users.rust-lang.org"))
                .resource("/", |r| r.f(|r| HttpResponse::Ok())),
            App::new().resource("/", |r| r.f(|r| HttpResponse::Ok())),
        ]
    });

    server.run();
}
// </vh>
