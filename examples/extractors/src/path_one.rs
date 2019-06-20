// <path-one>
use actix_web::{web, App, HttpServer, Result};

/// extract path info from "/users/{userid}/{friend}" url
/// {userid} -  - deserializes to a u32
/// {friend} - deserializes to a String
fn index(info: web::Path<(u32, String)>) -> Result<String> {
    Ok(format!("Welcome {}, userid {}!", info.1, info.0))
}

pub fn main() {
    HttpServer::new(|| {
        App::new().route(
            "/users/{userid}/{friend}", // <- define path parameters
            web::get().to(index),
        )
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
// </path-one>
