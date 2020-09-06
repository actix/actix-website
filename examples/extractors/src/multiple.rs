// <multi>
use actix_web::web;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

async fn index(
    web::Path((user_id, friend)): web::Path<(u32, String)>,
    query: web::Query<Info>,
) -> String {
    format!(
        "Welcome {}, friend {}, user_id {}!",
        query.username, friend, user_id
    )
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new().route(
            "/users/{user_id}/{friend}", // <- define path parameters
            web::get().to(index),
        )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
// </multi>
