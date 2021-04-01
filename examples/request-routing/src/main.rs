use actix_web::{web, App, HttpRequest, Responder, get};

// <request-routing>
#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Hello from the index page."
}

async fn hello(path: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &path)
}

#[actix_web::main]
async fn main() {
    let app = App::new()
        .service(index)
        .route("/{name}", web::get().to(hello));
}
// </request-routing>
