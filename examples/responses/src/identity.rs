// <identity>
use actix_web::{
    http::ContentEncoding, middleware::BodyEncoding, HttpRequest, HttpResponse,
};

fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        // v- disable compression
        .encoding(ContentEncoding::Identity)
        .body("data")
}
// </identity>

use actix_web::{web, App};
pub fn main() {
    App::new().route("/", web::get().to(index));
}
