// <guard>
use actix_web::{
    dev::RequestHead, guard::Guard, http, web, App, HttpResponse, HttpServer,
};

struct ContentTypeHeader;

impl Guard for ContentTypeHeader {
    fn check(&self, req: &RequestHead) -> bool {
        req.headers().contains_key(http::header::CONTENT_TYPE)
    }
}

pub fn main() {
    HttpServer::new(|| {
        App::new().route(
            "/",
            web::route()
                .guard(ContentTypeHeader)
                .to(|| HttpResponse::Ok()),
        )
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
// </guard>
