// <pred>
use actix_web::{dev::RequestHead, guard::Guard, http, web, App, HttpResponse};

struct ContentTypeHeader;

impl Guard for ContentTypeHeader {
    fn check(&self, req: &RequestHead) -> bool {
        req.headers().contains_key(http::header::CONTENT_TYPE)
    }
}

pub fn main() {
    App::new().route(
        "",
        web::route()
            .guard(ContentTypeHeader)
            .to(|| HttpResponse::Ok()),
    );
}
// </pred>
