// <brotli>
use actix_web::{
    http::ContentEncoding, middleware::BodyEncoding, HttpRequest, HttpResponse,
};

fn index_br(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .encoding(ContentEncoding::Br)
        .body("data")
}
// </brotli>

fn main() {}
