use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[allow(dead_code)]
async fn index(_req: HttpRequest) -> impl Responder {
    "Welcome!"
}

// <default>
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(|| HttpResponse::MethodNotAllowed()),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
// </default>
