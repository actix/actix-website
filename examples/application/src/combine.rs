use actix_web::{web, App, HttpResponse, HttpServer};

// <combine>
struct State1;
struct State2;

#[rustfmt::skip]
pub fn main() {
    HttpServer::new(|| {
        App::new()
            .data(State1)
            .data(State2)
            .service(
                web::scope("/app1")
                    .route("/", web::to(|| HttpResponse::Ok())))
            .service(
                web::scope("/app2")
                    .route("/", web::to(|| HttpResponse::Ok())))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
// </combine>
