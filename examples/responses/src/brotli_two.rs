// <brotli-two>
use actix_web::{http::ContentEncoding, middleware::BodyEncoding, HttpResponse};

fn index_br() -> HttpResponse {
    HttpResponse::Ok().body("data")
}

pub fn main() {
    use actix_web::{middleware, web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::new(ContentEncoding::Br))
            .route("/", web::get().to(index_br))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
// </brotli-two>
