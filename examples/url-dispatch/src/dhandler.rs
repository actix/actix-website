// <default>
use actix_web::{http::Method, pred, App, HttpResponse};

fn main() {
    App::new()
        .default_resource(|r| {
            r.method(Method::GET).f(|req| HttpResponse::NotFound());
            r.route()
                .filter(pred::Not(pred::Get()))
                .f(|req| HttpResponse::MethodNotAllowed());
        })
        .finish();
}
// </default>
