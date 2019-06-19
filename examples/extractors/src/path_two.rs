// <path-two>
use actix_web::{web, App, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    userid: u32,
    friend: String,
}

/// extract path info using serde
fn index(info: web::Path<Info>) -> Result<String> {
    Ok(format!("Welcome {}, userid {}!", info.friend, info.userid))
}

pub fn main() {
    App::new().route(
        "/users/{userid}/{friend}", // <- define path parameters
        web::get().to(index),
    );
}
// </path-two>
