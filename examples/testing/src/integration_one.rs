use actix_web::{HttpRequest, Responder};

#[allow(dead_code)]
fn index(_req: HttpRequest) -> impl Responder {
    "Hello world!"
}

// <integration-one>
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{test, web, App};

    #[test]
    fn test_index_get() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index)));
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::block_on(app.call(req)).unwrap();

        assert!(resp.status().is_success());
    }

    #[test]
    fn test_index_post() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index)));
        let req = test::TestRequest::post().uri("/").to_request();
        let resp = test::block_on(app.call(req)).unwrap();

        assert!(resp.status().is_client_error());
    }
}
// </integration-one>
