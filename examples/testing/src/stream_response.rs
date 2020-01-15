// <stream-response>
use std::task::Poll;
use bytes::Bytes;
use futures::stream::poll_fn;

use actix_web::http::{ContentEncoding, StatusCode};
use actix_web::{web, http, App, Error, HttpRequest, HttpResponse};

async fn sse(_req: HttpRequest) -> HttpResponse {
    let mut counter: usize = 5;

    // yields `data: N` where N in [5; 1]
    let server_events = poll_fn(move |_cx| -> Poll<Option<Result<Bytes, Error>>> {
        if counter == 0 {
            return Poll::Ready(None);
        }
        let payload = format!("data: {}\n\n", counter);
        counter -= 1;
        Poll::Ready(Some(Ok(Bytes::from(payload))))
    });

    HttpResponse::build(StatusCode::OK)
        .set_header(http::header::CONTENT_TYPE, "text/event-stream")
        .set_header(
            http::header::CONTENT_ENCODING,
            ContentEncoding::Identity.as_str(),
        )
        .streaming(server_events)
}

pub fn main() {
    App::new().route("/", web::get().to(sse));
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt;

    use futures_util::stream::StreamExt;
    use futures_util::stream::TryStreamExt;

    use actix_web::{test, web, App};

    #[actix_rt::test]
    async fn test_stream() {
        let mut app = test::init_service(App::new().route("/", web::get().to(sse))).await;
        let req = test::TestRequest::get().to_request();

        let mut resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        // first chunk
        let (bytes, mut resp) = resp.take_body().into_future().await;
        assert!(
            bytes.unwrap().unwrap() == Bytes::from_static(
                b"data: 5\n\n"
            )
        );

        // second chunk
        let (bytes, mut resp) = resp.take_body().into_future().await;
        assert!(
            bytes.unwrap().unwrap() == Bytes::from_static(
                b"data: 4\n\n"
            )
        );

        // remaining part
        let bytes = test::load_stream(resp.take_body().into_stream()).await;
        assert!(
            bytes.unwrap() == Bytes::from_static(
                b"data: 3\n\ndata: 2\n\ndata: 1\n\n"
            )
        );
    }
}
// </stream-response>
