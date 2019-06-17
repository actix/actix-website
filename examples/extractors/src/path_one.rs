// <path-one>
use actix_web::{web, App, Result};

/// extract path info from "/users/{userid}/{friend}" url
/// {userid} -  - deserializes to a u32
/// {friend} - deserializes to a String
fn index(info: web::Path<(u32, String)>) -> Result<String> {
    Ok(format!("Welcome {}! {}", info.1, info.0))
}

pub fn main() {
    App::new().route(
        "/users/{userid}/{friend}", // <- define path parameters
        web::get().to(index),
    );
}
// </path-one>
