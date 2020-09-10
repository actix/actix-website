#![allow(dead_code)]

// <norm>
use actix_web::{middleware, HttpResponse};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::default())
            .route("/resource/", web::to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
// </norm>
