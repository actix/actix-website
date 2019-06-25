// <resource>
use actix_web::{guard, web, App, HttpResponse};

fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

pub fn main() {
    App::new()
        .service(web::resource("/prefix").to(index))
        .service(
            web::resource("/user/{name}")
                .name("user_detail")
                .guard(guard::Header("content-type", "application/json"))
                .route(web::get().to(|| HttpResponse::Ok()))
                .route(web::put().to(|| HttpResponse::Ok())),
        );
}
// </resource>
