// <auto>
use actix_web::{http::ContentEncoding, middleware, HttpResponse};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("data")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::new(ContentEncoding::Br))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
// </auto>
