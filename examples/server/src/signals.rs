// <signals>
use actix_rt::System;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::sync::mpsc;
use std::thread;

pub fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let sys = System::new("http-server");

        let addr = HttpServer::new(|| {
            App::new().route("/", web::get().to(|| HttpResponse::Ok()))
        })
        .bind("127.0.0.1:8088")
        .unwrap()
        .shutdown_timeout(60) // <- Set shutdown timeout to 60 seconds
        .run();

        let _ = tx.send(addr);
    });

    let addr = rx.recv().unwrap();
    let _ = System::new("`actix_server::ServerCommand::Pause`")
        .block_on(addr.pause());
    let _ = System::new("`actix_server::ServerCommand::Resume`")
        .block_on(addr.resume());
    let _ = System::new("`actix_server::ServerCommand::Stop`")
        .block_on(addr.stop(true));
}
// </signals>
