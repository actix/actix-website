// <request-routing>
use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Hello from the index page."
}

async fn hello(path: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &path)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .route("/{name}", web::get().to(hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </request-routing>
