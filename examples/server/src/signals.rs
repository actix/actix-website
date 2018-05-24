use actix;
use futures::Future;

// <signals>
use actix_web::{server, App, HttpResponse};
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let sys = actix::System::new("http-server");
        let addr = server::new(|| {
            App::new()
                .resource("/", |r| r.f(|_| HttpResponse::Ok()))
        })
            .bind("127.0.0.1:0").expect("Can not bind to 127.0.0.1:0")
            .shutdown_timeout(60)    // <- Set shutdown timeout to 60 seconds
            .start();
        let _ = tx.send(addr);
        let _ = sys.run();
    });

    let addr = rx.recv().unwrap();
    let _ = addr.send(server::StopServer { graceful: true }).wait(); // <- Send `StopServer` message to server.
}
// </signals>
