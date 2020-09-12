use actix_web::{get, web, App, HttpRequest, Responder};

#[get("/show")]
async fn show_users(_req: HttpRequest) -> impl Responder {
    "unimplemented!"
}

// <scope>
#[actix_web::main]
async fn main() {
    let scope = web::scope("/users").service(show_users);
    App::new().service(scope);
}
// </scope>
