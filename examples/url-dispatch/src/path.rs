// <path>
use actix_web::{web, Result};

async fn index(info: web::Path<(String, u32)>) -> Result<String> {
    Ok(format!("Welcome {}! id: {}", info.0, info.1))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new().route(
            "/{username}/{id}/index.html", // <- define path parameters
            web::get().to(index),
        )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
// </path>
