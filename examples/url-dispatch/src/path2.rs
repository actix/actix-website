// <path>
use actix_web::{web, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

// extract path info using serde
async fn index(info: web::Path<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new().route(
            "/{username}/index.html", // <- define path parameters
            web::get().to(index),
        )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
// </path>
