// <stream>
use actix_web::{web, App, HttpServer, Error, HttpResponse};
use bytes::Bytes;
use futures::stream::once;
use futures::future::ok;

async fn index() -> HttpResponse {
    let body = once(ok::<_, Error>(Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/async", web::to(index)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
// </stream>
