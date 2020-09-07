// <stream>
use actix_web::{get, App, Error, HttpResponse, HttpServer};
use bytes::Bytes;
use futures::future::ok;
use futures::stream::once;

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(stream))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
// </stream>
