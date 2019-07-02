// <either>
use actix_web::{Either, Error, HttpResponse};
use futures::future::{ok, Future};

type RegisterResult =
    Either<HttpResponse, Box<Future<Item = HttpResponse, Error = Error>>>;

fn index() -> RegisterResult {
    if is_a_variant() {
        // <- choose variant A
        Either::A(HttpResponse::BadRequest().body("Bad data"))
    } else {
        Either::B(
            // <- variant B
            Box::new(ok(HttpResponse::Ok()
                .content_type("text/html")
                .body("Hello!".to_string()))),
        )
    }
}

fn main() {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
// </either>

fn is_a_variant() -> bool {
    true
}
