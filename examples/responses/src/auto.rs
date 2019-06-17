// <auto>
use actix_web::{
    http::ContentEncoding, middleware, web, App, HttpRequest, HttpResponse,
};

fn index(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("data")
}

fn main() {
    let app = App::new()
        // v- disable compression for all routes
        .wrap(middleware::Compress::new(ContentEncoding::Identity))
        .route("/", web::get().to(index));
}
// </auto>
