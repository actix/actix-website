pub mod keep_alive;
// pub mod keep_alive_tp;
pub mod signals;
pub mod ssl;
pub mod workers;

// <main>
use actix_web::{web, App, HttpResponse, HttpServer};

fn main() {
    let sys = actix_rt::System::new("example");

    HttpServer::new(|| {
        App::new().route("/", web::get().to(|| HttpResponse::Ok()))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .start();

    let _ = sys.run();
}
// </main>
