// <main-example>
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(greet))
            .service(web::resource("/{name}").to(greet))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
// </main-example>
