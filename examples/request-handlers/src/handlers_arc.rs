// <arc>
use actix_web::{dev::Handler, server, App, HttpRequest, HttpResponse};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

struct MyHandler(Arc<AtomicUsize>);

impl<S> Handler<S> for MyHandler {
    type Result = HttpResponse;

    /// Handle request
    fn handle(&self, _req: &HttpRequest<S>) -> Self::Result {
        self.0.fetch_add(1, Ordering::Relaxed);
        HttpResponse::Ok().into()
    }
}

pub fn main() {
    let sys = actix::System::new("example");

    let inc = Arc::new(AtomicUsize::new(0));

    server::new(move || {
        let cloned = inc.clone();
        App::new().resource("/", move |r| r.h(MyHandler(cloned)))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .start();

    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
}
// </arc>
