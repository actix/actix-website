// <json-one>
use actix_web::{web, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// deserialize `Info` from request's body
async fn index(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}
// </json-one>

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().route("/", web::post().to(index)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
