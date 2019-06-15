mod handlers_arc;
// <main>
use actix_web::{dev::Handler, server, App, HttpRequest, HttpResponse};
use std::cell::Cell;

struct MyHandler(Cell<usize>);

impl<S> Handler<S> for MyHandler {
    type Result = HttpResponse;

    /// Handle request
    fn handle(&self, _req: &HttpRequest<S>) -> Self::Result {
        let i = self.0.get();
        self.0.set(i + 1);
        HttpResponse::Ok().into()
    }
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.h(MyHandler(Cell::new(0))))) //use r.h() to bind handler, not the r.f()
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
// </main>
