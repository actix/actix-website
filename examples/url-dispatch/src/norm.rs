// <norm>
use actix_web::{middleware, web, App, HttpResponse};

fn main() {
    App::new()
        .wrap(middleware::NormalizePath)
        .route("/", web::get().to(|| HttpResponse::Ok()));
}
// </norm>

use actix_web::HttpRequest;
fn index(_req: HttpRequest) -> String {
    unimplemented!()
}
