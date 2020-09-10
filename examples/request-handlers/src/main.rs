pub mod handlers_arc;
// <data>
use actix_web::{web, Responder};
use std::cell::Cell;

#[derive(Clone)]
struct AppState {
    count: Cell<i32>,
}

async fn show_count(data: web::Data<AppState>) -> impl Responder {
    format!("count: {}", data.count.get())
}

async fn add_one(data: web::Data<AppState>) -> impl Responder {
    let count = data.count.get();
    data.count.set(count + 1);

    format!("count: {}", data.count.get())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    let data = AppState {
        count: Cell::new(0),
    };

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .route("/", web::to(show_count))
            .route("/add", web::to(add_one))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
// </data>
