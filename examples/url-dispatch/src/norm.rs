// <norm>
use actix_web::{middleware, web, App};

pub fn main() {
    App::new()
        .wrap(middleware::NormalizePath)
        .route("/", web::get().to(index));
}
// </norm>

use actix_web::HttpRequest;
fn index(_req: HttpRequest) -> String {
    unimplemented!()
}
