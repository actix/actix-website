use actix_rt;
use futures::Future;

// <signals>
use actix_web::{web, App, HttpResponse, HttpServer};
use std::sync::mpsc;
use std::thread;

pub fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let sys = actix_rt::System::new("http-server");

        let addr = HttpServer::new(|| {
            App::new().route("/", web::get().to(|| HttpResponse::Ok()))
        })
        .bind("127.0.0.1:8088")
        .unwrap()
        .shutdown_timeout(60) // <- Set shutdown timeout to 60 seconds
        .start();

        let _ = tx.send(addr);
        let _ = sys.run();
    });

    let addr = rx.recv().unwrap();
    let _ = addr
        .pause()
        .wait()
        .map(|_| println!("`actix_server::ServerCommand::Pause`"));
    let _ = addr
        .resume()
        .wait()
        .map(|_| println!("`actix_server::ServerCommand::Resume`"));
    let _ = addr
        .stop(true)
        .wait()
        .map(|_| println!("`actix_server::ServerCommand::Stop`"));
}
// </signals>
