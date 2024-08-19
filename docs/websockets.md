---
title: WebSockets
---

import CodeBlock from "@site/src/components/code_block";

# WebSockets

Actix Web supports a high-level WebSocket interface via the `actix-ws` crate. Using this crate, it's possible to convert a request's `Payload` stream into a stream of [_ws::Message_][message]s and then react to them inside a spawned async task.

The following is an example of a simple WebSocket echo server:

<CodeBlock example="websockets" file="main.rs" section="websockets" />

> A simple WebSocket echo server example is available [in the examples repo][echo].

> An example chat server is also available [in the examples directory][chat]

[message]: https://docs.rs/actix-ws/0.3/actix_ws/enum.Message.html
[payload]: https://docs.rs/actix-web/4/actix_web/web/struct.Payload.html
[echo]: https://github.com/actix/examples/tree/master/websockets/echo-actorless
[chat]: https://github.com/actix/examples/tree/master/websockets/chat-actorless
