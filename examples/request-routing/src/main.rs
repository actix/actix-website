// <request-routing>
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

fn index(_req: HttpRequest) -> impl Responder {
    "Hello from the index page."
}

fn hello(path: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &path)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index))
            .service(web::resource("/{name}").to(hello))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
// </request-routing>
