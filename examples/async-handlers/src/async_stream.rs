fn is_error() -> bool {
    false
}

// <async-stream>
use actix_web::{error, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use futures::future::{result, Future};

fn index(
    _req: HttpRequest,
) -> Result<Box<Future<Item = HttpResponse, Error = Error>>, Error> {
    if is_error() {
        Err(error::ErrorBadRequest("bad request"))
    } else {
        Ok(Box::new(result(Ok(HttpResponse::Ok()
            .content_type("text/html")
            .body("Hello!")))))
    }
}
// </async-stream>

pub fn main() {
    HttpServer::new(|| App::new().route("/", web::to_async(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
