// <arc>
use actix_web::{get, web, App, HttpServer, Responder};
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    thread_count: Cell<usize>,
    total_count: Arc<AtomicUsize>,
}

#[get("/")]
async fn show_count(data: web::Data<AppState>) -> impl Responder {
    format!(
        "total_count: {}\nthread_count: {}",
        data.total_count.load(Ordering::Relaxed),
        data.thread_count.get()
    )
}

#[get("/add")]
async fn add_one(data: web::Data<AppState>) -> impl Responder {
    data.total_count.fetch_add(1, Ordering::Relaxed);

    let thread_count = data.thread_count.get();
    data.thread_count.set(thread_count + 1);

    format!(
        "total_count: {}\nthread_count: {}",
        data.total_count.load(Ordering::Relaxed),
        data.thread_count.get()
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = AppState {
        thread_count: Cell::new(0),
        total_count: Arc::new(AtomicUsize::new(0)),
    };

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .service(show_count)
            .service(add_one)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
// </arc>
