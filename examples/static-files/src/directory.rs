// <directory>
use actix_files as fs;
use actix_web::{App, HttpServer};

pub fn main() {
    HttpServer::new(|| {
        App::new().service(fs::Files::new("/static", ".").show_files_listing())
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
// </directory>
