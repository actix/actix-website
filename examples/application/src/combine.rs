#![allow(dead_code)]

use actix_web::{web, App, HttpResponse, HttpServer};

// <combine>
struct State1;
struct State2;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/app1")
                    .app_data(web::Data::new(State1))
                    .route("/", web::to(HttpResponse::Ok)),
            )
            .service(
                web::scope("/app2")
                    .app_data(web::Data::new(State2))
                    .route("/", web::to(HttpResponse::Ok)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </combine>
