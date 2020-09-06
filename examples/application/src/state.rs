// <setup>
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

// This struct represents state
struct AppState {
    app_name: String,
}

async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name

    format!("Hello {}!", app_name) // <- response with app_name
}
// </setup>

// <start_app>
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
// </start_app>
