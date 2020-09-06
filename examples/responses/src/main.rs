pub mod auto;
pub mod brotli;
pub mod chunked;
pub mod compress;
pub mod identity;
pub mod identity_two;
pub mod json_resp;

// <builder>
use actix_web::HttpResponse;

async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("plain/text")
        .header("X-Hdr", "sample")
        .body("data")
}
// </builder>

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
