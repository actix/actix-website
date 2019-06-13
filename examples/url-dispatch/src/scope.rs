use actix_web::{App, HttpRequest, HttpResponse};

// <scope>
fn show_users(_req: HttpRequest) -> HttpResponse {
    unimplemented!()
}

#[rustfmt::skip]
fn main() {
    App::new().service(
        web::scope("/users") .route("/show", web::to(show_users)))
}
// </scope>
