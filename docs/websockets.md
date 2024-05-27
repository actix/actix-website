---
title: Websockets
---

import CodeBlock from "@site/src/components/code_block";

# Websockets

Actix Web supports WebSockets with the `actix-web-actors` crate. It is possible to convert a request's `Payload` to a stream of [_ws::Message_][message] with a [_web::Payload_][payload] and then use stream combinators to handle actual messages, but it is simpler to handle websocket communications with an http actor.

The following is an example of a simple websocket echo server:

<CodeBlock example="websockets" file="main.rs" section="websockets" />

> A simple websocket echo server example is available in the [examples directory][examples].

> An example chat server with the ability to chat over a websocket or TCP connection is available in [websocket-chat directory][chat]

[message]: https://docs.rs/actix-web-actors/2/actix_web_actors/ws/enum.Message.html
[payload]: https://docs.rs/actix-web/4/actix_web/web/struct.Payload.html
[examples]: https://github.com/actix/examples/tree/master/websockets
[chat]: https://github.com/actix/examples/tree/master/websockets/chat
