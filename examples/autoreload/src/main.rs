// <autoreload>
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use listenfd::ListenFd;

fn index(_req: HttpRequest) -> impl Responder {
    "Hello World!"
}

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().route("/", web::get().to(index)));

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:3000").unwrap()
    };

    server.run().unwrap();
}
// </autoreload>
