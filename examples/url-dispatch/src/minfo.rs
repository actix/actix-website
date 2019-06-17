// <minfo>
use actix_web::{web, App, HttpRequest, HttpServer, Result};

fn index(req: HttpRequest) -> Result<String> {
    let v1: u8 = req.match_info().get("v1").unwrap().parse().unwrap();
    let v2: u8 = req.match_info().query("v2").parse().unwrap();
    let (v3, v4): (u8, u8) = req.match_info().load().unwrap();
    Ok(format!("Values {} {} {} {}", v1, v2, v3, v4))
}

fn main() {
    App::new()
        .route("/a/{v1}/{v2}/", web::get().to(index))
        .route("", web::get().to(|| actix_web::HttpResponse::Ok()));
}
// </minfo>
