// <web-socket>
use actix::{Actor, StreamHandler};
use actix_web::*;
use futures::Stream;

struct Ws; // <- WebSocket actor

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for Ws {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Text(text) => ctx.text(text),
            _ => (),
        }
    }
}

fn main() {
    let mut srv = test::TestServer::new(|app| app.handler(|req| ws::start(req, Ws)));

    let (reader, mut writer) = srv.ws().unwrap();
    writer.text("text");

    let (item, reader) = srv.execute(reader.into_future()).unwrap();
    assert_eq!(item, Some(ws::Message::Text("text".to_owned())));
}
// </web-socket>
