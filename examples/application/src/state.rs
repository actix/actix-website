// <setup>
use actix_web::{web, App, HttpServer};
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

// <make_app>
fn _main() {
    App::new()
        .data(AppState {
            counter: Cell::new(0),
        })
        .route("/", web::get().to(index));
}
// </make_app>

// <start_app>
pub fn main() {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                counter: Cell::new(0),
            })
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
// </start_app>
