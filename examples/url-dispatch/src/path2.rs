// <path>
extern crate serde_derive;
use actix_web::{http::Method, App, Path, Result};

#[derive(Deserialize)]
struct Info {
    username: String,
}

// extract path info using serde
fn index(info: Path<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

fn main() {
    let app = App::new().resource(
        "/{username}/index.html", // <- define path parameters
        |r| r.method(Method::GET).with(index),
    );
}
// </path>
