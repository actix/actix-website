// <keep-alive>
use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let one = HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
        .keep_alive(75); // <- Set keep-alive to 75 seconds

    // let _two = HttpServer::new(|| {
    //     App::new().route("/", web::get().to(|| HttpResponse::Ok()))
    // })
    // .keep_alive(); // <- Use `SO_KEEPALIVE` socket option.

    let _three = HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
        .keep_alive(None); // <- Disable keep-alive

    one.bind(("127.0.0.1", 8080))?.run().await
}
// </keep-alive>
