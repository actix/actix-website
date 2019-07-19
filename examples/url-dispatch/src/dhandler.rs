use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

fn index(_req: HttpRequest) -> impl Responder {
    "Welcome!"
}

// <default>
pub fn main() {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(|| HttpResponse::MethodNotAllowed()),
            )
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
// </default>
