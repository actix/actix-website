// <request-routing>
use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hello, World!"
}

async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").get(index))
            .service(web::resource("/{name}").get(hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </request-routing>
