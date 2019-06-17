// <config-one>
// extern crate actix_web;
// extern crate mime;
// use actix_files::{FileConfig, NamedFile};
// use actix_web::http::header::DispositionType;
// use actix_web::{http::Method, App, HttpRequest, Result};

// use std::path::PathBuf;

// #[derive(Default)]
// struct MyConfig;

// impl FileConfig for MyConfig {
//     fn content_disposition_map(typ: mime::Name) -> DispositionType {
//         DispositionType::Attachment
//     }
// }

// fn index(req: &HttpRequest) -> Result<NamedFile<MyConfig>> {
//     let path: PathBuf = req.match_info().query("tail")?;
//     Ok(NamedFile::open_with_config(path, MyConfig)?)
// }

// fn main() {
//     App::new().resource(r"/a/{tail:.*}", |r| r.method(Method::GET).f(index));
// }
// </config-one>
