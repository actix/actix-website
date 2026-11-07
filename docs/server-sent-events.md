---
title: Server-Sent Events (SSE)
sidebar_label: Server-Sent Events (SSE)
description: >-
  Stream server-sent events with Actix Web and receive updates in a browser with EventSource.
---

# Server-Sent Events (SSE)

Server-sent events (SSE) let a server send updates to a client over a long-lived HTTP response. Use SSE for notifications, progress updates, or live feeds. Communication is one-way: the server sends events to the client. For two-way communication, see [WebSockets](websockets.md).

Actix Web supports SSE through the [`sse` module][sse] in the `actix-web-lab` crate. Its [`Sse`][responder] responder sets the `text/event-stream` content type and formats events for the client.

## Send Events

Create an `Sse` responder from a stream with `Sse::from_stream()`, or from a Tokio channel receiver with `Sse::from_infallible_receiver()`. Use [`Data`][data] to create messages. Each message can include an event name and an ID.

For a long-lived connection, use `with_keep_alive()` to send periodic comments while no events are available. Use `with_retry_duration()` to set the client's reconnection delay.

See the [SSE API examples][sse] for handlers that use a stream or a channel. A complete [broadcast example][example] is available in the examples repository.

## Receive Events in a Browser

Use the browser's [`EventSource`][eventsource] API to connect to an SSE endpoint:

```javascript
const events = new EventSource("/events");

events.onmessage = (event) => {
  console.log(event.data);
};
```

Replace `/events` with your endpoint's path. The `message` handler receives events that have no event name. For named events, use `addEventListener()` with the matching name:

```javascript
events.addEventListener("chat_msg", (event) => {
  console.log(event.data);
});
```

The browser reconnects automatically if the connection closes. Call `events.close()` when updates are no longer needed. Event IDs let the browser send a `Last-Event-ID` header when it reconnects; your application must handle that ID if it needs to replay missed events.

[sse]: https://docs.rs/actix-web-lab/latest/actix_web_lab/sse/index.html
[responder]: https://docs.rs/actix-web-lab/latest/actix_web_lab/sse/struct.Sse.html
[data]: https://docs.rs/actix-web-lab/latest/actix_web_lab/sse/struct.Data.html
[example]: https://github.com/actix/examples/tree/master/server-sent-events
[eventsource]: https://developer.mozilla.org/en-US/docs/Web/API/EventSource
