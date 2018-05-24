// <cfg>
use actix_web::{pred, App, HttpResponse};

fn main() {
    App::new()
        .resource("/path", |resource| {
            resource
                .route()
                .filter(pred::Get())
                .filter(pred::Header("content-type", "text/plain"))
                .f(|req| HttpResponse::Ok())
        })
        .finish();
}
// </cfg>
