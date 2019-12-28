// <setup>
use actix_web::{web, App, Responder};

async fn index() -> impl Responder {
    "Hello world!"
}

#[rustfmt::skip]
#[actix_rt::main]
async fn main() {
    App::new().service(
        web::scope("/app")
            .route("/index.html", web::get().to(index)));
}
// </setup>
