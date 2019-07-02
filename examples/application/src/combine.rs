use actix_web::{web, App, HttpResponse, HttpServer};

// <combine>
struct State1;
struct State2;

#[rustfmt::skip]
pub fn main() {
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
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
// </combine>
