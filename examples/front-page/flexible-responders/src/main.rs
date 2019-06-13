use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use serde::Serialize;

// <flexible-responders>
#[derive(Serialize)]
struct Measurement {
    temperature: f32,
}

fn hello_world() -> impl Responder {
    "Hello World!"
}

fn current_temperature(_req: HttpRequest) -> impl Responder {
    web::Json(Measurement { temperature: 42.3 })
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(hello_world))
            .service(web::resource("/temp").to(current_temperature))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();
}
// </flexible-responders>