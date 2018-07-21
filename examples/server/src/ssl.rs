// <ssl>
use actix_web::{server, App, HttpRequest, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

fn index(req: &HttpRequest) -> impl Responder {
    "Welcome!"
}

fn main() {
    // load ssl keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    server::new(|| App::new().resource("/index.html", |r| r.f(index)))
        .bind_ssl("127.0.0.1:8080", builder)
        .unwrap()
        .run();
}
// </ssl>
