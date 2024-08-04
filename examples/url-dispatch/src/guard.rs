#![allow(dead_code)] // false positive on ContentTypeHeader

// <guard>
use actix_web::{
    guard::{Guard, GuardContext},
    http, HttpResponse,
};

struct ContentTypeHeader;

impl Guard for ContentTypeHeader {
    fn check(&self, req: &GuardContext) -> bool {
        req.head()
            .headers()
            .contains_key(http::header::CONTENT_TYPE)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new().route(
            "/",
            web::route().guard(ContentTypeHeader).to(HttpResponse::Ok),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </guard>
