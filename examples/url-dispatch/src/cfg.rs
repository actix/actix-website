// <cfg>
use actix_web::{guard, web, App, HttpResponse};

pub fn main() {
    App::new().service(
        web::resource("/").route(
            web::route()
                .guard(guard::Get())
                .guard(guard::Header("content-type", "text/plain"))
                .to(|| HttpResponse::Ok()),
        ),
    );
}
// </cfg>
