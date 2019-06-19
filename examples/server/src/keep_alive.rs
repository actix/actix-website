// <keep-alive>
use actix_web::{web, App, HttpResponse, HttpServer};

pub fn main() {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(|| HttpResponse::Ok()))
    })
    .keep_alive(75); // <- Set keep-alive to 75 seconds

    HttpServer::new(|| {
        App::new().route("/", web::get().to(|| HttpResponse::Ok()))
    })
    .keep_alive(server::KeepAlive::Tcp(75)); // <- Use `SO_KEEPALIVE` socket option.

    HttpServer::new(|| {
        App::new().route("/", web::get().to(|| HttpResponse::Ok()))
    })
    .keep_alive(None); // <- Disable keep-alive
}
// </keep-alive>
