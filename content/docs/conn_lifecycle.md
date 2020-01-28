---
title: Connection Lifecycle
menu: docs_architecture
weight: 20
---


# Architecture overview

After Server has started listening to all sockets, [`Accept`][Accept] and [`Worker`][Worker] are two main loops responsible for processing incoming client connections. 

Once connection accepted Application level protocol processing happens in a protocol specific [`Dispatcher`][Dispatcher] loop spawned from [`Worker`][Worker].

    Please note, below diagrams are outlining happy-path scenarios only.

![](/img/diagrams/connection_overview.svg)

## Accept loop in more detail

![](/img/diagrams/connection_accept.svg)

Most of code implementation resides in [`actix-server`][server] crate for struct [`Accept`][Accept]. 

## Worker loop in more detail

![](/img/diagrams/connection_worker.svg)

Most of code implementation resides in [`actix-server`][server] crate for struct [`Worker`][Worker]. 

## Request loop roughly

![](/img/diagrams/connection_request.svg)

Most of code implementation for request loop resides in [`actix-web`][web] and [`actix-http`][http] crates. 


[server]: https://crates.io/crates/actix-server
[web]: https://crates.io/crates/actix-web
[http]: https://crates.io/crates/actix-http
[Accept]: https://github.com/actix/actix-net/blob/master/actix-server/src/accept.rs
[Worker]: https://github.com/actix/actix-net/blob/master/actix-server/src/worker.rs
[Dispatcher]: https://github.com/actix/actix-web/blob/master/actix-http/src/h1/dispatcher.rs