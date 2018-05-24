// <setup>
use actix_web::{http, App, HttpRequest};
use std::cell::Cell;

// This struct represents state
struct AppState {
    counter: Cell<usize>,
}

fn index(req: HttpRequest<AppState>) -> String {
    let count = req.state().counter.get() + 1; // <- get count
    req.state().counter.set(count); // <- store new count in state

    format!("Request number: {}", count) // <- response with count
}
// </setup>

fn make_app() {
// <make_app>
App::with_state(AppState { counter: Cell::new(0) })
    .resource("/", |r| r.method(http::Method::GET).f(index))
    .finish()
// </make_app>
;
}

use actix_web::{server, HttpResponse};
use std::thread;

fn combine() {
    thread::spawn(|| {
// <combine>
struct State1;
struct State2;

fn main() {
    server::new(|| {
        vec![
            App::with_state(State1)
                .prefix("/app1")
                .resource("/", |r| r.f(|r| HttpResponse::Ok()))
                .boxed(),
            App::with_state(State2)
                .prefix("/app2")
                .resource("/", |r| r.f(|r| HttpResponse::Ok()))
                .boxed(),
                ]
    }).bind("127.0.0.1:8080")
        .unwrap()
        .run()
}
// </combine>
    });
}

pub fn test() {
    make_app();
    combine();
}
