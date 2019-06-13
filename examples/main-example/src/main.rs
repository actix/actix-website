// <main-example>
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

fn greet(req: HttpRequest) -> HttpResponse {
    let name = req.match_info().get("name").unwrap_or("World");
    HttpResponse::Ok().body(format!("Hello {}!", &name))
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
