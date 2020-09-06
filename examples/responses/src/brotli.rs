// <brotli>
use actix_web::{http::ContentEncoding, dev::BodyEncoding, HttpResponse};

async fn index_br() -> HttpResponse {
    HttpResponse::Ok()
        .encoding(ContentEncoding::Br)
        .body("data")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{middleware, web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .route("/", web::get().to(index_br))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
// </brotli>
