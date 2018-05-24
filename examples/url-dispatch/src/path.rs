// <path>
use actix_web::{http::Method, App, Path, Result};

// extract path info using serde
fn index(info: Path<(String, u32)>) -> Result<String> {
    Ok(format!("Welcome {}! id: {}", info.0, info.1))
}

fn main() {
    let app = App::new().resource(
        "/{username}/{id}/index.html", // <- define path parameters
        |r| r.method(Method::GET).with(index),
    );
}
// </path>
