#![allow(unused)]
use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

// <vh>
fn main() {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "www.rust-lang.org"))
                    .route("", web::to(|| HttpResponse::Ok())),
            )
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "users.rust-lang.org"))
                    .route("", web::to(|| HttpResponse::Ok())),
            )
            .route("/", web::to(|| HttpResponse::Ok()))
    })
    .run();
}
// </vh>
