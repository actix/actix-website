// <query>
use actix_web::{get, web, App, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

// this handler get called only if the request's query contains `username` field
#[get("/")]
async fn index(info: web::Query<Info>) -> String {
    format!("Welcome {}!", info.username)
}
// </query>

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
