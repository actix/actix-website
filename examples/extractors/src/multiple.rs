// <multi>
use actix_web::{get, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn index(path: web::Path<(u32, String)>, query: web::Query<Info>) -> String {
    let (user_id, friend) = path.into_inner();
    format!(
        "Welcome {}, friend {}, user_id {}!",
        query.username, friend, user_id
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
// </multi>
