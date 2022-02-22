// <stream-response>
use std::task::Poll;

use actix_web::{
    http::{self, header::ContentEncoding, StatusCode},
    web, App, Error, HttpRequest, HttpResponse,
};
use futures::stream;

async fn sse(_req: HttpRequest) -> HttpResponse {
    let mut counter: usize = 5;

    // yields `data: N` where N in [5; 1]
    let server_events =
        stream::poll_fn(move |_cx| -> Poll<Option<Result<web::Bytes, Error>>> {
            if counter == 0 {
                return Poll::Ready(None);
            }
            let payload = format!("data: {}\n\n", counter);
            counter -= 1;
            Poll::Ready(Some(Ok(web::Bytes::from(payload))))
        });

    HttpResponse::build(StatusCode::OK)
        .insert_header((http::header::CONTENT_TYPE, "text/event-stream"))
        .insert_header(ContentEncoding::Identity)
        .streaming(server_events)
}

pub fn main() {
    App::new().route("/", web::get().to(sse));
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::{
        body::{self, MessageBody as _},
        rt::pin,
        test, web, App,
    };
    use futures::future;

    #[actix_web::test]
    async fn test_stream() {
        let app = test::init_service(App::new().route("/", web::get().to(sse))).await;
        let req = test::TestRequest::get().to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = resp.into_body();
        pin!(body);

        // first chunk
        let bytes = future::poll_fn(|cx| body.as_mut().poll_next(cx)).await;
        assert_eq!(
            bytes.unwrap().unwrap(),
            web::Bytes::from_static(b"data: 5\n\n")
        );

        // second chunk
        let bytes = future::poll_fn(|cx| body.as_mut().poll_next(cx)).await;
        assert_eq!(
            bytes.unwrap().unwrap(),
            web::Bytes::from_static(b"data: 4\n\n")
        );

        // TODO: fix this example
        // // remaining part
        // let bytes = body::to_bytes(body).await;
        // assert_eq!(
        //     bytes.unwrap(),
        //     web::Bytes::from_static(b"data: 3\n\ndata: 2\n\ndata: 1\n\n")
        // );
    }
}
// </stream-response>
