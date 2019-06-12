// // <setup>
// extern crate actix_web;
// use actix_web::{server, App, HttpRequest};

// fn index(_req: &HttpRequest) -> &'static str {
//     "Hello world!"
// }
// // </setup>
// // <main>
// fn main() {
//     server::new(|| App::new().resource("/", |r| r.f(index)))
//         .bind("127.0.0.1:8088")
//         .unwrap()
//         .run();
// }
// // </main>

// <setup>
extern crate actix_web;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}
// </setup>
// <main>
fn main() {
    HttpServer::new(|| App::new().service(web::resource("/").to(index)))
        .bind("127.0.0.1:8000")
        .unwrap()
        .run()
        .unwrap();
}
// </main>
