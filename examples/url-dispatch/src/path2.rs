// <path>
use actix_web::{http::Method, web, App, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

// extract path info using serde
fn index(info: web::Path<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

fn main() {
    let app = App::new().resource(
        "/{username}/index.html", // <- define path parameters
        |r| r.method(Method::GET).with(index),
    );
}
// </path>
