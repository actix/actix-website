pub mod auto;
pub mod brotli;
pub mod chunked;
pub mod identity;
pub mod identity_two;
pub mod json_resp;

// <builder>
use actix_web::{http::ContentEncoding, middleware::BodyEncoding, HttpResponse};

fn index() -> HttpResponse {
    HttpResponse::Ok()
        .encoding(ContentEncoding::Br)
        .content_type("plain/text")
        .header("X-Hdr", "sample")
        .body("data")
}
// </builder>

pub fn main() {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
