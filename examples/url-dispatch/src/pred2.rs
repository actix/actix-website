// <pred>
use actix_web::{pred, App, HttpResponse};

fn main() {
    App::new()
        .resource("/index.html", |r| {
            r.route()
                .filter(pred::Not(pred::Get()))
                .f(|req| HttpResponse::MethodNotAllowed())
        })
        .finish();
}
// </pred>
