// <chunked>
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use futures::future::ok;
use futures::stream::once;

#[get("/")]
async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().streaming(once(ok::<_, Error>(web::Bytes::from_static(b"data"))))
}
// </chunked>

#[actix_web::main]
async fn main() {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run();
}
