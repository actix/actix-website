// <either>
use actix_web::{web, App, Either, Error, HttpRequest, HttpResponse};
use futures::future::{ok, Future};

type RegisterResult =
    Either<HttpResponse, Box<Future<Item = HttpResponse, Error = Error>>>;

fn index(_req: HttpRequest) -> RegisterResult {
    if is_a_variant() {
        // <- choose variant A
        Either::A(HttpResponse::BadRequest().body("Bad data"))
    } else {
        Either::B(
            // <- variant B
            Box::new(ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(format!("Hello!")))),
        )
    }
}

fn main() {
    App::new().route("/", web::get().to(index));
}
// </either>

fn is_a_variant() -> bool {
    true
}
