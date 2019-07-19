// <identity>
use actix_web::{
    http::ContentEncoding, middleware, middleware::BodyEncoding, HttpResponse,
};

fn index() -> HttpResponse {
    HttpResponse::Ok()
        // v- disable compression
        .encoding(ContentEncoding::Identity)
        .body("data")
}

pub fn main() {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
// </identity>
