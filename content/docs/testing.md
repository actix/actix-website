---
title: Testing
menu: docs_advanced
weight: 210
---

# Testing

Every application should be well tested. Actix-web provides tools to perform unit and
integration tests.

# Unit Tests

For unit testing, actix-web provides a request builder type and a simple handler runner.
[*TestRequest*][testrequest] implements a builder-like pattern.  You can generate a
`HttpRequest` instance with `to_http_request()`, or you can
run your handler with `block_on()`.

{{< include-example example="testing" file="main.rs" section="unit-tests" >}}

# Integration tests

There a few methods for testing your application. Actix-web can be used
to run the application with specific handlers in a real http server.

`TestRequest::get()`, `TestRequest::post()`, and `TestRequest::client()`
methods can be used to send requests to the test server.

To create a `Service` for testing, use the `test::init_service` method which accepts a
regular `App` builder.

> Check the [api documentation][actixdocs] for more information.

{{< include-example example="testing" file="integration_one.rs" section="integration-one" >}}

Note: If you get the error message "no Task is currently running" when running your
test, you may need to wrap your service call in
[test::run_on](https://docs.rs/actix-web/1/actix_web/test/fn.run_on.html):

```rust
let future = test::run_on(|| app.call(req));
let resp = test::block_on(future).unwrap();
```

If you need more complex application configuration testing should be very similar to creating
the normal application. For example, you may need to initialize application state. Create an
`App` with a `data` method and attach state just like you would from a normal application.

{{< include-example example="testing" file="integration_two.rs" section="integration-two" >}}

# Stream response tests

If you need to test stream it would be enough to convert a [*ClientResponse*][clientresponse]
to future and execute it.
For example of testing [*Server Sent Events*][serversentevents].

{{< include-example example="testing" file="stream_response.rs" section="stream-response" >}}

[serversentevents]: https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events
[clientresponse]: https://docs.rs/actix-web/1.0.2/actix_web/client/struct.ClientResponse.html
[actixdocs]: (https://docs.rs/actix-web/1.0.2/actix_web/test/index.html)
[testrequest]: https://docs.rs/actix-web/1.0.2/actix_web/error/trait.ResponseError.html#foreign-impls
