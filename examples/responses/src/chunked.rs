// <chunked>
use actix_web::{web, HttpRequest, HttpResponse, Error};
use bytes::Bytes;
use futures::future::ok;
use futures::stream::once;

async fn index(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .streaming(once(ok::<_, Error>(Bytes::from_static(b"data"))))
}
// </chunked>

pub fn main() {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
