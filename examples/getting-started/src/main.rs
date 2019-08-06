// <setup>
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}
// </setup>

// <macro-attributes>
use actix_web::get;

#[get("/hello")]
fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
// </macro-attributes>

// <main>
fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
// </main>
