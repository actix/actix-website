use actix_web::{web, App, HttpResponse, HttpServer};

// <scope>
fn show_users() -> HttpResponse {
    HttpResponse::Ok().body("Show users")
}

fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User detail: {}", path.0))
}

pub fn main() {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/users")
                .route("/show", web::get().to(show_users))
                .route("/show/{id}", web::get().to(user_detail)),
        )
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
// </scope>
