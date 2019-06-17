// <multi>
use actix_web::{web, App};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

fn index((_path, query): (web::Path<(u32, String)>, web::Query<Info>)) -> String {
    format!("Welcome {}!", query.username)
}

pub fn main() {
    App::new().route(
        "/users/{userid}/{friend}", // <- define path parameters
        web::get().to(index),
    );
}
// </multi>
