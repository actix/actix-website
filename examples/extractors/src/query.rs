// <query>
use actix_web::{web, App};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

// this handler get called only if the request's query contains `username` field
fn index(info: web::Query<Info>) -> String {
    format!("Welcome {}!", info.username)
}

pub fn main() {
    App::new().route("/", web::get().to(index));
}
// </query>
