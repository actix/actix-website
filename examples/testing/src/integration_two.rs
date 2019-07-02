use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct AppState {
    count: i32,
}

#[allow(dead_code)]
fn index(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.get_ref())
}

// <integration-two>
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[test]
    fn test_index_get() {
        let mut app = test::init_service(
            App::new()
                .data(AppState { count: 4 })
                .route("/", web::get().to(index)),
        );
        let req = test::TestRequest::get().uri("/").to_request();
        let resp: AppState = test::read_response_json(&mut app, req);

        assert!(resp.count == 4);
    }
}
// </integration-two>
