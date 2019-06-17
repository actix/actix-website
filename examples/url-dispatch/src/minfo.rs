// <minfo>
use actix_web::{web, App, HttpRequest, Result};

fn index(req: HttpRequest) -> Result<String> {
    let v1: u8 = req.match_info().query("v1").parse().unwrap();
    let v2: u8 = req.match_info().query("v2").parse().unwrap();
    Ok(format!("Values {} {}", v1, v2))
}

fn main() {
    App::new().route(r"/a/{v1}/{v2}/", web::get().to(index));
}
// </minfo>
