pub mod json_two;
pub mod manual;
pub mod multipart;
pub mod streaming;
pub mod urlencoded;
// <json-request>
use actix_web::{web, App, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// extract `Info` using serde
fn index(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

fn main() {
    App::new().route("/index.html", web::post().to(index));
}
// </json-request>
