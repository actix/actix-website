#![allow(dead_code, unused)]
// <setup>
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use std::cell::Cell;

// This struct represents state
struct AppState {
    counter: Cell<usize>,
}

fn index(data: web::Data<AppState>) -> String {
    let count = data.counter.get() + 1; // <- get count
    data.counter.set(count); // <- store new count in state

    format!("Request number: {}", count) // <- response with count
}
// </setup>

#[rustfmt::skip]
fn make_app() {
// <make_app>
App::new()
    .data( AppState { counter: Cell::new(0) })
    .route("/", web::get().to(index));
// </make_app>
}

#[rustfmt::skip]
fn start_app() {
// <start_app>
HttpServer::new(|| {
    App::new()
        .data( AppState { counter: Cell::new(0) })
        .route("/", web::get().to(index))
})
.bind("127.0.0.1:8088")
.unwrap()
.run()
.unwrap();
// </start_app>
}

use std::thread;

#[rustfmt::skip]
fn combine() {
    thread::spawn(|| {
// <combine>
struct State1;
struct State2;

fn main() {
    HttpServer::new(|| {
        App::new()
            .data(State1)
            .data(State2)
            .service(
                web::scope("/app1")
                    .route("/", web::to(|| HttpResponse::Ok())),
            )
            .service(
                web::scope("/app2")
                    .route("/", web::to(|| HttpResponse::Ok())),
            )
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
// </combine>
    });
}

pub fn test() {
    make_app();
    combine();
}
