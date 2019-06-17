mod configuration;
mod configuration_two;
mod directory;
// <individual-file>
use actix_files::NamedFile;
use actix_web::{web, App, HttpRequest, Result};
use std::path::PathBuf;

fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("tail").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

fn main() {
    App::new().route("/", web::get().to(index));
}
// </individual-file>
