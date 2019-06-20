pub mod async_stream;
pub mod stream;
// <async-responder>
use actix_web::{web, App, Error, HttpRequest, HttpResponse};
use futures::future::{ok, Future};

fn index(_req: HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    Box::new(ok::<_, Error>(
        HttpResponse::Ok().content_type("text/html").body("Hello!"),
    ))
}

fn index2(_req: HttpRequest) -> Box<Future<Item = &'static str, Error = Error>> {
    Box::new(ok::<_, Error>("Welcome!"))
}

fn main() {
    App::new()
        .route("/async", web::to_async(index))
        .route("/", web::to_async(index2));
}
// </async-responder>
