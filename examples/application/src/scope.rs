use actix_web::{web, App, HttpRequest, Responder};

async fn show_users(_req: HttpRequest) -> impl Responder {
    "unimplemented!"
}

#[rustfmt::skip]
// <scope>
#[actix_rt::main]
async fn main() {
    App::new()
        .service(
            web::scope("/users")
                .route("/show", web::get().to(show_users)));
}
// </scope>
