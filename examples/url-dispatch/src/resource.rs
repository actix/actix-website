// <resource>
use actix_web::{http::Method, App, HttpRequest, HttpResponse};

fn index(req: &HttpRequest) -> HttpResponse {
    unimplemented!()
}

fn main() {
    App::new()
        .resource("/prefix", |r| r.f(index))
        .resource("/user/{name}", |r| {
            r.method(Method::GET).f(|req| HttpResponse::Ok())
        })
        .finish();
}
// </resource>
