#![allow(unused_variables)]
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

mod state;
mod vh;

#[rustfmt::skip]
fn make_app() {

// <make_app>
fn index(_req: HttpRequest) -> impl Responder {
    "Hello world!"
}

let app = App::new()
    .service(web::scope("/app").route("/index.html", web::get().to(index)));
// </make_app>

}

#[rustfmt::skip]
fn run_server() {
// <run_server>
let server = HttpServer::new(|| {
    App::new()
        .service(web::scope("/app1").route("/", web::to(|| HttpResponse::Ok())))
        .service(web::scope("/app2").route("/", web::to(|| HttpResponse::Ok())))
        .route("/", web::to(|| HttpResponse::Ok()))
});
// </run_server>
}

fn main() {
    make_app();
    run_server();
    state::test();
}
