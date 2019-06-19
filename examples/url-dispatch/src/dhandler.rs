use actix_web::{guard, web, App, HttpRequest, HttpResponse, Responder};

fn index(_req: HttpRequest) -> impl Responder {
    "Welcome!"
}

// <default>
pub fn main() {
    App::new()
        .service(web::resource("/").route(web::get().to(index)))
        .default_service(
            web::route()
                .guard(guard::Not(guard::Get()))
                .to(|| HttpResponse::MethodNotAllowed()),
        );
}
// </default>
