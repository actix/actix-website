// <integration-one>
// use actix_web::test::TestServer;
// use actix_web::HttpRequest;
// use std::str;

// fn index(req: HttpRequest) -> &'static str {
//     "Hello world!"
// }

// fn main() {
//     // start new test server
//     let mut srv = TestServer::new(|app| app.handler(index));

//     let request = srv.get().finish().unwrap();
//     let response = srv.execute(request.send()).unwrap();
//     assert!(response.status().is_success());

//     let bytes = srv.execute(response.body()).unwrap();
//     let body = str::from_utf8(&bytes).unwrap();
//     assert_eq!(body, "Hello world!");
// }
// </integration-one>
