---
title: Databases
---

import CodeBlock from "@site/src/components/code_block";

# Async Options

We have several example projects showing use of async database adapters:

- [Postgres](https://github.com/actix/examples/tree/master/databases/postgres)
- [SQLite](https://github.com/actix/examples/tree/master/databases/sqlite)
- [MongoDB](https://github.com/actix/examples/tree/master/databases/mongodb)

# Diesel

The current versions of Diesel (v1/v2) does not support asynchronous operations, so it is important to use the [`web::block`][web-block] function to offload your database operations to the Actix runtime thread-pool.

You can create action functions that correspond to all the operations your app will perform on the database.

<CodeBlock example="databases" file="main.rs" section="handler" />

Now you should set up the database pool using a crate such as `r2d2`, which makes many DB connections available to your app. This means that multiple handlers can manipulate the DB at the same time, and still accept new connections. Simply, the pool in your app state. (In this case, it's beneficial not to use a state wrapper struct because the pool handles shared access for you.)

<CodeBlock example="databases" file="main.rs" section="main" />

Now, in a request handler, use the `Data<T>` extractor to get the pool from app state and get a connection from it. This provides an owned database connection that can be passed into a [`web::block`][web-block] closure. Then just call the action function with the necessary arguments and `.await` the result.

This example also maps the error to an `HttpResponse` before using the `?` operator but this is not necessary if your return error type implements [`ResponseError`][response-error].

<CodeBlock example="databases" file="main.rs" section="index" />

That's it! See the full example [here](https://github.com/actix/examples/tree/master/databases/diesel).

# SeaORM

[SeaORM](https://www.sea-ql.org/SeaORM/) is a Rust ORM with full async support. When used with Actix Web, unlike Diesel, it allows you to perform database operations directly in an asynchronous manner without needing to use `web::block`.

First, define your data models and functions for database operations. Using `sea-orm-cli`, you can use a data model that is automatically generated from an existing DB definition:

<CodeBlock example="sea-orm-databases" file="main.rs" section="handler" />

Next, set up the database connection as part of your application state. SeaORM manages connection pools by default, so you don't need additional pool configuration:

<CodeBlock example="sea-orm-databases" file="main.rs" section="main" />

In your request handler, use the `web::Data<DatabaseConnection>` extractor to get the database connection and perform async operations directly:

<CodeBlock example="sea-orm-databases" file="main.rs" section="index" />

For a full example, please refer to [here](https://github.com/actix/examples/tree/master/databases/sea-orm).

[web-block]: https://docs.rs/actix-web/4/actix_web/web/fn.block.html
[response-error]: https://docs.rs/actix-web/4/actix_web/error/trait.ResponseError.html
