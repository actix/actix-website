use actix_web::{web, App, HttpResponse, HttpServer};

// <combine>
struct State1;
struct State2;

#[rustfmt::skip]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/app1")
                    .data(State1)
                    .route("/", web::to(|| HttpResponse::Ok())))
            .service(
                web::scope("/app2")
                    .data(State2)
                    .route("/", web::to(|| HttpResponse::Ok())))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
// </combine>
