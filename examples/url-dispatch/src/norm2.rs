use actix_web::HttpResponse;

#[get("/resource/")]
fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

// <norm>
use actix_web::{get, http::Method, middleware, web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath)
            .service(index)
            .default_service(web::route().method(Method::GET))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
// </norm>
