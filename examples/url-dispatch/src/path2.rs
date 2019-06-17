// <path>
use actix_web::{web, App, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

// extract path info using serde
fn index(info: web::Path<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

pub fn main() {
    App::new().route(
        "/{username}/index.html", // <- define path parameters
        web::get().to(index),
    );
}
// </path>
