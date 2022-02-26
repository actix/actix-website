use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct AppState {
    count: i32,
}

#[allow(dead_code)]
async fn index(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.get_ref())
}

// <integration-two>
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState { count: 4 }))
                .route("/", web::get().to(index)),
        )
        .await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp: AppState = test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.count, 4);
    }
}
// </integration-two>
