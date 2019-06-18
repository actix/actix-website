// <setup>
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
// </setup>

// <main>
fn main() {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
// </main>
