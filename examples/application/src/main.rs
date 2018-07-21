#![allow(unused)]
extern crate actix_web;
use actix_web::{http::Method, server, App, HttpRequest, HttpResponse, Responder};

mod state;
mod vh;

fn make_app() {
// <make_app>
fn index(req: &HttpRequest) -> impl Responder {
    "Hello world!"
}

let app = App::new()
    .prefix("/app")
    .resource("/index.html", |r| r.method(Method::GET).f(index))
    .finish()
// </make_app>
;
}

fn run_server() {
// <run_server>
let server = server::new(|| {
    vec![
        App::new()
            .prefix("/app1")
            .resource("/", |r| r.f(|r| HttpResponse::Ok())),
        App::new()
            .prefix("/app2")
            .resource("/", |r| r.f(|r| HttpResponse::Ok())),
        App::new().resource("/", |r| r.f(|r| HttpResponse::Ok())),
    ]
});
// </run_server>
}

fn main() {
    make_app();
    run_server();
    state::test();
}
