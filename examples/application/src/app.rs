// <setup>
use actix_web::{web, App, Responder};

fn index() -> impl Responder {
    "Hello world!"
}

#[rustfmt::skip]
pub fn main() {
    App::new().service(
        web::scope("/app")
            .route("/index.html", web::get().to(index)));
}
// </setup>
