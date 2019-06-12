// <request-routing>
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Hello from the index page.")
}

fn hello(path: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Hello {}!", &path))
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index))
            .service(web::resource("/{name}").to(hello))
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
    .unwrap();
}
// </request-routing>
