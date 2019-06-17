mod auto;
mod brotli;
mod chunked;
mod identity;
mod identity_two;
mod json_resp;
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

fn main() {}
