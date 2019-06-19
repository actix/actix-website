// <auto>
use actix_web::{
    http::ContentEncoding, middleware, web, App, HttpRequest, HttpResponse,
};

fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("data")
}

pub fn main() {
    App::new()
        // v- disable compression for all routes
        .wrap(middleware::Compress::new(ContentEncoding::Identity))
        .route("/", web::get().to(index));
}
// </auto>
