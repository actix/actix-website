// <stream>
use actix_web::{Error, HttpResponse};
use bytes::Bytes;
use futures::stream::once;

fn index() -> HttpResponse {
    let body = once::<Bytes, Error>(Ok(Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(Box::new(body))
}

pub fn main() {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/async", web::to_async(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
// </stream>
