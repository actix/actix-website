// <resource>
use actix_web::{http::Method, web, App, HttpRequest, HttpResponse};

fn index(_req: HttpRequest) -> HttpResponse {
    unimplemented!()
}

fn main() {
    App::new()
        .service(web::resource("/prefix").to(index))
        .service(
            web::resource("/user/{name}").route(web::get().to(|| HttpResponse::Ok())),
        );
}
// </resource>
