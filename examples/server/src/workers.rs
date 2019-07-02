// <workers>
use actix_web::{web, App, HttpResponse, HttpServer};

pub fn main() {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(|| HttpResponse::Ok()))
    })
    .workers(4); // <- Start 4 workers
}
// </workers>
