// <norm>
use actix_web::{http::NormalizePath, App};

fn main() {
    let app = App::new()
        .resource("/resource/", |r| r.f(index))
        .default_resource(|r| r.h(NormalizePath::default()))
        .finish();
}
// </norm>

use actix_web::HttpRequest;
fn index(req: HttpRequest) -> String {
    unimplemented!()
}
