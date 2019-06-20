// <query>
use actix_web::{web, App, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

// this handler get called only if the request's query contains `username` field
fn index(info: web::Query<Info>) -> String {
    format!("Welcome {}!", info.username)
}
// </query>

pub fn main() {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
