// <chunked>
use actix_web::{get, App, Error, HttpRequest, HttpResponse, HttpServer};
use bytes::Bytes;
use futures::future::ok;
use futures::stream::once;

#[get("/")]
async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().streaming(once(ok::<_, Error>(Bytes::from_static(b"data"))))
}
// </chunked>

#[actix_web::main]
async fn main() {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8000")
        .unwrap()
        .run();
}
