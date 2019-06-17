---
title: Testing
menu: docs_advanced
weight: 210
---

# Testing

Every application should be well tested. Actix provides tools to perform unit and
integration tests.

# Unit Tests

For unit testing, actix provides a request builder type and a simple handler runner.
[*TestRequest*](../../actix-web/actix_web/test/struct.TestRequest.html)
implements a builder-like pattern.
You can generate a `HttpRequest` instance with `to_http_request()`, or you can
run your handler with `block_on()`.

{{< include-example example="testing" file="main.rs" section="unit-tests" >}}

# Integration tests

There are several methods for testing your application. Actix provides
[*TestServer*](../../actix-web/actix_web/test/struct.TestServer.html), which can be used
to run the application with specific handlers in a real http server.

`TestServer::get()`, `TestServer::post()`, and `TestServer::client()`
methods can be used to send requests to the test server.

A simple form `TestServer` can be configured to use a handler.
`TestServer::new` method accepts a configuration function, and the only argument 
for this function is a *test application* instance.

> Check the [api documentation](../../actix-web/actix_web/test/struct.TestApp.html)
> for more information.

{{< include-example example="testing" file="integration_one.rs" section="integration-one" >}}

The other option is to use an application factory. In this case, you need to pass the factory
function the same way as you would for real http server configuration.

{{< include-example example="testing" file="integration_two.rs" section="integration-two" >}}

If you need more complex application configuration, use the `TestServer::build_with_state()`
method. For example, you may need to initialize application state or start `SyncActor`'s for diesel
interation. This method accepts a closure that constructs the application state,
and it runs when the actix system is configured. Thus, you can initialize any additional actors.

{{< include-example example="testing" file="integration_three.rs" section="integration-three" >}}

# Stream response tests

If you need to test stream it would be enough to convert a [*ClientResponse*](../../actix-web/actix_web/client/struct.ClientResponse.html) to future and execute it.
For example of testing [*Server Sent Events*](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events).

{{< include-example example="testing" file="stream_response.rs" section="stream-response" >}}

# WebSocket server tests

It is possible to register a *handler* with `TestApp::handler()`, which
initiates a web socket connection. *TestServer* provides the method `ws()`, which connects to
the websocket server and returns ws reader and writer objects. *TestServer* also
provides an `execute()` method, which runs future objects to completion and returns
result of the future computation.

The following example demonstrates how to test a websocket handler:

{{< include-example example="testing" file="websockets.rs" section="web-socket" >}}
