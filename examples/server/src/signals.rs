// <signals>
use actix_web::{web, App, HttpResponse, HttpServer};
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let srv = HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
        .bind(("127.0.0.1", 8080))?
        .shutdown_timeout(60) // <- Set shutdown timeout to 60 seconds
        .run();

    // obtain handle to server
    let srv_handle = srv.handle();

    // spawn server as Tokio task to start processing connections
    tokio::spawn(srv);

    // pause accepting new connections
    srv_handle.pause().await;

    // resume accepting new connections
    srv_handle.resume().await;

    // stop server gracefully
    srv_handle.stop(true).await;

    Ok(())
}
// </signals>

#[allow(dead_code)]
fn run_main() {
    let _ = main();
}
