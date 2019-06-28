// <path>
use actix_web::{web, Result};

fn index(info: web::Path<(String, u32)>) -> Result<String> {
    Ok(format!("Welcome {}! id: {}", info.0, info.1))
}

pub fn main() {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new().route(
            "/{username}/{id}/index.html", // <- define path parameters
            web::get().to(index),
        )
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
// </path>
