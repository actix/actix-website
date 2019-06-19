// <resource>
use actix_web::{web, App, HttpRequest, HttpResponse};

fn index(_req: HttpRequest) -> HttpResponse {
    unimplemented!()
}

pub fn main() {
    App::new()
        .service(web::resource("/prefix").to(index))
        .service(
            web::resource("/user/{name}").route(web::get().to(|| HttpResponse::Ok())),
        );
}
// </resource>
