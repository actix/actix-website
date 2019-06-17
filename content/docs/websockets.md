---
title: Websockets
menu: docs_proto
weight: 240
---

Actix supports WebSockets out-of-the-box. It is possible to convert a request's `Payload`
to a stream of [*ws::Message*](../../actix-web/actix_web/ws/enum.Message.html) with
a [*ws::WsStream*](../../actix-web/actix_web/ws/struct.WsStream.html) and then use stream
combinators to handle actual messages, but it is simpler to handle websocket communications
with an http actor.

The following is an example of a simple websocket echo server:

{{< include-example example="websockets" file="main.rs" section="websockets" >}}

> A simple websocket echo server example is available in the
> [examples directory](https://github.com/actix/examples/tree/master/websocket/).

> An example chat server with the ability to chat over a websocket or tcp connection
> is available in [websocket-chat directory](https://github.com/actix/examples/tree/master/websocket-chat/)
