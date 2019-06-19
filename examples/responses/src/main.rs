pub mod auto;
pub mod brotli;
pub mod chunked;
pub mod identity;
pub mod identity_two;
pub mod json_resp;
// <builder>
use actix_web::{
    http::ContentEncoding, middleware::BodyEncoding, HttpRequest, HttpResponse,
};

fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .encoding(ContentEncoding::Br)
        .content_type("plain/text")
        .header("X-Hdr", "sample")
        .body("data")
}
// </builder>

use actix_web::{web, App};
pub fn main() {
    App::new().route("/", web::get().to(index));
}
