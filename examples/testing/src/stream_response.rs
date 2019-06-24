// <stream-response>
use bytes::Bytes;
use futures::stream::poll_fn;
use futures::{Async, Poll};

use actix_web::http::{ContentEncoding, StatusCode};
use actix_web::{middleware::BodyEncoding, web, App, Error, HttpRequest, HttpResponse};

fn sse(_req: HttpRequest) -> HttpResponse {
    let mut counter: usize = 5;

    // yields `data: N` where N in [5; 1]
    let server_events = poll_fn(move || -> Poll<Option<Bytes>, Error> {
        if counter == 0 {
            return Ok(Async::Ready(None));
        }
        let payload = format!("data: {}\n\n", counter);
        counter -= 1;
        Ok(Async::Ready(Some(Bytes::from(payload))))
    });

    HttpResponse::build(StatusCode::OK)
        .encoding(ContentEncoding::Identity)
        .content_type("text/event-stream")
        .streaming(server_events)
}

pub fn main() {
    App::new().route("/", web::get().to(sse));
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[test]
    fn test_stream() {
        let mut app = test::init_service(App::new().route("/", web::get().to(sse)));
        let req = test::TestRequest::get().to_request();
        let resp = test::read_response(&mut app, req);
        assert!(
            resp == Bytes::from_static(
                b"data: 5\n\ndata: 4\n\ndata: 3\n\ndata: 2\n\ndata: 1\n\n"
            )
        );
    }
}
// </stream-response>
