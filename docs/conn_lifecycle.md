---
title: Connection Lifecycle
---

# Architecture overview

After Server has started listening to all sockets, [`Accept`][accept] and [`Worker`][worker] are two main loops responsible for processing incoming client connections.

Once connection accepted Application level protocol processing happens in a protocol specific [`Dispatcher`][dispatcher] loop spawned from [`Worker`][worker].

    Please note, below diagrams are outlining happy-path scenarios only.

![Connection Overview](/img/diagrams/connection_overview.svg)

## Accept loop in more detail

![Connection Accept](/img/diagrams/connection_accept.svg "Connection Accept")

Most of code implementation resides in [`actix-server`][server] crate for struct [`Accept`][accept].

## Worker loop in more detail

![Connection Worker](/img/diagrams/connection_worker.svg "Connection Worker")

Most of code implementation resides in [`actix-server`][server] crate for struct [`Worker`][worker].

## Request loop roughly

![Connection Request](/img/diagrams/connection_request.svg "Connection Request")

Most of code implementation for request loop resides in [`actix-web`][web] and [`actix-http`][http] crates.

[server]: https://crates.io/crates/actix-server
[web]: https://crates.io/crates/actix-web
[http]: https://crates.io/crates/actix-http
[accept]: https://github.com/actix/actix-net/blob/master/actix-server/src/accept.rs
[worker]: https://github.com/actix/actix-net/blob/master/actix-server/src/worker.rs
[dispatcher]: https://github.com/actix/actix-web/blob/master/actix-http/src/h1/dispatcher.rs
