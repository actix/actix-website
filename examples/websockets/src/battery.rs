use actix_web::{Error, HttpRequest, HttpResponse, rt, web};

#[expect(unused_variables)]
// <battery>
#[rustfmt::skip]
pub async fn websocket(
    req: HttpRequest,
    pl: web::Payload,
) -> Result<HttpResponse, Error> {
    let (res, mut session, incoming) = actix_ws::handle(&req, pl)?;

    rt::spawn(async move {
        session.text("Hello, WebSocket!").await?;
        session.close(None).await
    });

    Ok(res)
}
// </battery>
