use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;

// <flexible-responders>
#[derive(Serialize)]
struct Measurement {
    temperature: f32,
}

async fn hello_world() -> impl Responder {
    "Hello World!"
}

async fn current_temperature() -> impl Responder {
    web::Json(Measurement { temperature: 42.3 })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(hello_world))
            .service(web::resource("/temp").to(current_temperature))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
// </flexible-responders>
