// <multi>
use actix_web::web;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

fn index((path, query): (web::Path<(u32, String)>, web::Query<Info>)) -> String {
    format!(
        "Welcome {}, friend {}, useri {}!",
        query.username, path.1, path.0
    )
}

pub fn main() {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new().route(
            "/users/{userid}/{friend}", // <- define path parameters
            web::get().to(index),
        )
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
// </multi>
