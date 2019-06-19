// <pred>
use actix_web::{guard, web, App, HttpResponse};

pub fn main() {
    App::new().route(
        "/",
        web::route()
            .guard(guard::Not(guard::Get()))
            .to(|| HttpResponse::MethodNotAllowed()),
    );
}
// </pred>
