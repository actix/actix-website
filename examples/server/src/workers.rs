// <workers>
use actix_web::{server::HttpServer, App, HttpResponse};

fn main() {
    HttpServer::new(|| App::new().resource("/", |r| r.f(|_| HttpResponse::Ok())))
        .workers(4); // <- Start 4 workers
}
// </workers>
