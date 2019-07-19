// <auto>
use actix_web::{http::ContentEncoding, middleware, HttpResponse};

fn index() -> HttpResponse {
    HttpResponse::Ok().body("data")
}

pub fn main() {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::new(ContentEncoding::Br))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
// </auto>
