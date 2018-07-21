use actix_web::{App, HttpRequest, HttpResponse};

// <prefix>
fn show_users(req: &HttpRequest) -> HttpResponse {
    unimplemented!()
}

fn main() {
    App::new()
        .prefix("/users")
        .resource("/show", |r| r.f(show_users))
        .finish();
}
// </prefix>
