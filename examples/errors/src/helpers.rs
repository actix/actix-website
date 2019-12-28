use actix_web::{web, App};
// <helpers>
use actix_web::{error, Result};

#[derive(Debug)]
struct MyError {
    name: &'static str,
}

async fn index() -> Result<&'static str> {
    let result: Result<&'static str, MyError> = Err(MyError { name: "test error" });

    Ok(result.map_err(|e| error::ErrorBadRequest(e.name))?)
}
// </helpers>

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::HttpServer;

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
