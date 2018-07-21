// <pred>
use actix_web::{http, server::Request, pred::Predicate, App, HttpResponse};

struct ContentTypeHeader;

impl<S: 'static> Predicate<S> for ContentTypeHeader {
    fn check(&self, req: &Request, state: &S) -> bool {
        req.headers().contains_key(http::header::CONTENT_TYPE)
    }
}

fn main() {
    App::new().resource("/index.html", |r| {
        r.route()
            .filter(ContentTypeHeader)
            .f(|_| HttpResponse::Ok())
    });
}
// </pred>
