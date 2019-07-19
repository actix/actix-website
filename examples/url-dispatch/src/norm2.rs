use actix_web::HttpResponse;

fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

// <norm>
use actix_web::{http::Method, middleware, web, App, HttpServer};

pub fn main() {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath)
            .route("/resource/", web::get().to(index))
            .default_service(web::route().method(Method::GET))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
// </norm>
