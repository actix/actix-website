---
title: Testing
menu: docs_advanced
weight: 215
---

# Testing

Every application should be well tested. Actix-web provides tools to perform unit and
integration tests.

# Unit Tests

For unit testing, actix-web provides a request builder type.
[*TestRequest*][testrequest] implements a builder-like pattern. You can generate a
`HttpRequest` instance with `to_http_request()` and call your handler with it.

{{< include-example example="testing" file="main.rs" section="unit-tests" >}}

# Integration tests

There a few methods for testing your application. Actix-web can be used
to run the application with specific handlers in a real http server.

`TestRequest::get()`, `TestRequest::post()` and other
methods can be used to send requests to the test server.

To create a `Service` for testing, use the `test::init_service` method which accepts a
regular `App` builder.

> Check the [api documentation][actixdocs] for more information.

{{< include-example example="testing" file="integration_one.rs" section="integration-one" >}}

If you need more complex application configuration testing should be very similar to creating
the normal application. For example, you may need to initialize application state. Create an
`App` with a `data` method and attach state just like you would from a normal application.

{{< include-example example="testing" file="integration_two.rs" section="integration-two" >}}

# Stream response tests

If you need to test stream it would be enough to call `take_body()` and convert a resulting [*ResponseBody*][responsebody]
to future and execute it.
For example of testing [*Server Sent Events*][serversentevents].

{{< include-example example="testing" file="stream_response.rs" section="stream-response" >}}

[serversentevents]: https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events
[responsebody]: https://docs.rs/actix-web/2/actix_web/body/enum.ResponseBody.html
[actixdocs]: https://docs.rs/actix-web/2/actix_web/test/index.html
[testrequest]: https://docs.rs/actix-web/2/actix_web/test/struct.TestRequest.html
