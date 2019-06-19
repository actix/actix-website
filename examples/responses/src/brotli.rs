// <brotli>
use actix_web::{
    http::ContentEncoding, middleware::BodyEncoding, HttpRequest, HttpResponse,
};

fn index_br(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .encoding(ContentEncoding::Br)
        .body("data")
}
// </brotli>

use actix_web::{web, App};
pub fn main() {
    App::new().route("/", web::get().to(index_br));
}
