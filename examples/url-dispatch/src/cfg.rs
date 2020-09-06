use actix_web::{guard, web, App, HttpResponse};

#[rustfmt::skip]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::HttpServer;

    HttpServer::new(|| {
// <cfg>
App::new().service(
    web::resource("/path").route(
        web::route()
            .guard(guard::Get())
            .guard(guard::Header("content-type", "text/plain"))
            .to(|| HttpResponse::Ok()),
    ),
)
// </cfg>
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
