// <path>
use actix_web::{web, App, Result};

// extract path info using serde
fn index(info: web::Path<(String, u32)>) -> Result<String> {
    Ok(format!("Welcome {}! id: {}", info.0, info.1))
}

pub fn main() {
    App::new().route(
        "/{username}/{id}/index.html", // <- define path parameters
        web::get().to(index),
    );
}
// </path>
