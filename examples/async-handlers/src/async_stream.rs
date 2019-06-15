fn is_error() -> bool {
    true
}

// <main>
use actix_web::{error, App, Error, HttpRequest, HttpResponse};
use futures::future::{result, Future};

fn index(
    _req: &HttpRequest,
) -> Result<Box<Future<Item = HttpResponse, Error = Error>>, Error> {
    if is_error() {
        Err(error::ErrorBadRequest("bad request"))
    } else {
        Ok(Box::new(result(Ok(HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("Hello!"))))))
    }
}
// </main>

pub fn main() {
    App::new().resource("/", |r| r.route().f(index)).finish();
}
