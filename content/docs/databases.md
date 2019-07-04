---
title: Databases
menu: docs_patterns
weight: 1010
---

# Diesel

{{% alert %}}
NOTE: The `actix-web` 1.0 version of this section is still
[being updated](https://github.com/cldershem/actix-website/tree/update1.0-db). Checkout
this [example](https://github.com/actix/examples/tree/master/async_db) until then.
{{% /alert %}}

[being updated](https://github.com/cldershem/actix-website/tree/update1.0-db).

At the moment, Diesel 1.0 does not support asynchronous operations,
but it's possible to use the `actix` synchronous actor system as a database interface api.

Technically, sync actors are worker style actors. Multiple sync actors
can be run in parallel and process messages from same queue. Sync actors work in mpsc mode.

Let's create a simple database api that can insert a new user row into a SQLite table.
We must define a sync actor and a connection that this actor will use. The same approach
can be used for other databases.

{{< include-example example="og_databases" file="main.rs" section="actor" >}}

This is the definition of our actor. Now, we must define the *create user* message and response.

{{< include-example example="og_databases" file="main.rs" section="message" >}}

We can send a `CreateUser` message to the `DbExecutor` actor, and as a result, we will receive a
`User` model instance. Next, we must define the handler implementation for this message.

{{< include-example example="og_databases" file="main.rs" section="handler" >}}

That's it! Now, we can use the *DbExecutor* actor from any http handler or middleware.
All we need is to start *DbExecutor* actors and store the address in a state where http handler
can access it.

{{< include-example example="og_databases" file="main.rs" section="main" >}}

We will use the address in a request handler. The handle returns a future object;
thus, we receive the message response asynchronously.
`Route::a()` must be used for async handler registration.

{{< include-example example="og_databases" file="main.rs" section="index" >}}

> A full example is available in the [examples directory][examples].

> More information on sync actors can be found in the
> [actix documentation][actixdocs].

[examples]: https://github.com/actix/examples/tree/master/diesel/
[actixdocs]: https://docs.rs/actix/0.7.0/actix/sync/index.html
