pub mod configuration;
pub mod configuration_two;
pub mod directory;

// <individual-file>
use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;

fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

fn main() {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/{filename:.*}", web::get().to(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
// </individual-file>
