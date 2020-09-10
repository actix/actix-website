// <auto>
use actix_web::{get, http::ContentEncoding, middleware, App, HttpResponse, HttpServer};

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("data")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::new(ContentEncoding::Br))
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
// </auto>
