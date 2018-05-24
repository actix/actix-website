// <ka>
use actix_web::{server, App, HttpResponse};

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(|_| HttpResponse::Ok())))
        .keep_alive(75); // <- Set keep-alive to 75 seconds

    server::new(|| App::new().resource("/", |r| r.f(|_| HttpResponse::Ok())))
        .keep_alive(server::KeepAlive::Tcp(75)); // <- Use `SO_KEEPALIVE` socket option.

    server::new(|| App::new().resource("/", |r| r.f(|_| HttpResponse::Ok())))
        .keep_alive(None); // <- Disable keep-alive
}
// </ka>
