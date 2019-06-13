// <setup>
extern crate actix_web;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}
// </setup>

// <main>
fn main() {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
// </main>
