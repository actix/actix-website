extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate openssl;

mod ka;
mod ka_tp;
mod signals;
mod ssl;
mod workers;

// <main>
use actix_web::{server::HttpServer, App, HttpResponse};

fn main() {
    let sys = actix::System::new("guide");

    HttpServer::new(|| App::new().resource("/", |r| r.f(|_| HttpResponse::Ok())))
        .bind("127.0.0.1:59080")
        .unwrap()
        .start();

    let _ = sys.run();
}
// </main>
