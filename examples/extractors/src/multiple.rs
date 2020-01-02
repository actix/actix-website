// <multi>
use actix_web::web;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

async fn index(
    (path, query): (web::Path<(u32, String)>, web::Query<Info>),
) -> String {
    format!(
        "Welcome {}, friend {}, userid {}!",
        query.username, path.1, path.0
    )
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new().route(
            "/users/{userid}/{friend}", // <- define path parameters
            web::get().to(index),
        )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
// </multi>
