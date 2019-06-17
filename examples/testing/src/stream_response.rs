// <stream-response>
// use bytes::Bytes;
// use futures::stream::poll_fn;
// use futures::{Async, Poll, Stream};

// use actix_web::http::{ContentEncoding, StatusCode};
// use actix_web::test::TestServer;
// use actix_web::{Error, HttpRequest, HttpResponse};

// fn sse(_req: &HttpRequest) -> HttpResponse {
//     let mut counter = 5usize;
//     // yields `data: N` where N in [5; 1]
//     let server_events = poll_fn(move || -> Poll<Option<Bytes>, Error> {
//         if counter == 0 {
//             return Ok(Async::Ready(None));
//         }
//         let payload = format!("data: {}\n\n", counter);
//         counter -= 1;
//         Ok(Async::Ready(Some(Bytes::from(payload))))
//     });

//     HttpResponse::build(StatusCode::OK)
//         .content_encoding(ContentEncoding::Identity)
//         .content_type("text/event-stream")
//         .streaming(server_events)
// }

// fn main() {
//     // start new test server
//     let mut srv = TestServer::new(|app| app.handler(sse));

//     // request stream
//     let request = srv.get().finish().unwrap();
//     let response = srv.execute(request.send()).unwrap();
//     assert!(response.status().is_success());

//     // convert ClientResponse to future, start read body and wait first chunk
//     let mut stream = response.payload();
//     loop {
//         match srv.execute(stream.into_future()) {
//             Ok((Some(bytes), remain)) => {
//                 println!("{:?}", bytes);
//                 stream = remain
//             }
//             Ok((None, _)) => break,
//             Err(_) => panic!(),
//         }
//     }
// }
// </stream-response>
