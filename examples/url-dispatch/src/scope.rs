use actix_web::{web, App, HttpRequest, HttpResponse};

// <scope>
fn show_users(_req: HttpRequest) -> HttpResponse {
    unimplemented!()
}

pub fn main() {
    App::new().service(web::scope("/users").route("/show", web::get().to(show_users)));
}
// </scope>
