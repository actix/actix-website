#![allow(unused)]
extern crate actix_web;
use actix_web::{http::Method, server, App, HttpRequest, HttpResponse, Responder};

mod state;

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
