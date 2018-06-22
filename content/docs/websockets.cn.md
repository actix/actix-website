---
title: Websockets
menu: docs_proto
weight: 240
---

Actix支持WebSockets开箱即用。可以使用[ws :: WsStream](https://actix.rs/actix-web/actix_web/ws/struct.WsStream.html)将请求的Payload转换为[ws :: Message](https://actix.rs/actix-web/actix_web/ws/enum.Message.html)流，然后使用流组合器来处理实际的消息，但处理websocket通信使用http actor更简单。

以下是一个简单的websocket echo server的例子：

```rust
use actix::*;
use actix_web::*;

/// Define http actor
struct Ws;

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<ws::Message, ws::ProtocolError> for Ws {

    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => ctx.text(text),
            ws::Message::Binary(bin) => ctx.binary(bin),
            _ => (),
        }
    }
}

fn main() {
    App::new()
      .resource("/ws/", |r| r.f(|req| ws::start(req, Ws)))
      .finish();
}
```

[websocket directory](https://github.com/actix/examples/tree/master/websocket/)提供了一个简单的websocket echo server示例 。

[websocket-chat directory](https://github.com/actix/examples/tree/master/websocket-chat/)提供了一个聊天服务器，可以通过websocket或tcp连接进行聊天
