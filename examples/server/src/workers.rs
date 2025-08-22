// <workers>
use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() {
    let _ = HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok))).workers(4);
    // <- Start 4 workers
}
// </workers>
