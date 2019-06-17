// <norm>
use actix_web::{http::Method, middleware, web, App};

fn main() {
    App::new()
        .wrap(middleware::NormalizePath)
        .route("/resource/", web::get().to(index))
        .default_service(web::route().method(Method::GET));
}
// </norm>

use actix_web::HttpRequest;

fn index(_req: HttpRequest) -> String {
    unimplemented!()
}
