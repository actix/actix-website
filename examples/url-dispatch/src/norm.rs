// <norm>
use actix_web::{middleware, web, App, HttpResponse};

fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

pub fn main() {
    use actix_web::HttpServer;

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath)
            .route("/resource/", web::to(index))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
// </norm>
