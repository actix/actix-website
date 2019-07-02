use actix_web::{web, App, HttpRequest, Responder};

fn show_users(_req: HttpRequest) -> impl Responder {
    unimplemented!()
}

#[rustfmt::skip]
// <scope>
pub fn main() {
    App::new()
        .service(
            web::scope("/users")
                .route("/show", web::get().to(show_users)));
}
// </scope>
