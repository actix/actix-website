// <integration-two>
// use actix_web::{http, test, App, HttpRequest, HttpResponse};

// fn index(req: &HttpRequest) -> HttpResponse {
//     HttpResponse::Ok().into()
// }

// /// This function get called by http server.
// fn create_app() -> App {
//     App::new().resource("/test", |r| r.h(index))
// }

// fn main() {
//     let mut srv = test::TestServer::with_factory(create_app);

//     let request = srv.client(http::Method::GET, "/test").finish().unwrap();
//     let response = srv.execute(request.send()).unwrap();

//     assert!(response.status().is_success());
// }
// </integration-two>
