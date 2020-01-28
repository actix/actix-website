---
title: Connection Lifecycle
menu: docs_architecture
weight: 20
---


# Architecture overview

After Server has started listening to all sockets, `Accept` and `Worker` are two main loops responsible for processing incoming client connections. 

Once connection accepted Application level protocol processing happens in a `Dispatcher` loop spawned from `Worker`.

    Please note, below diagrams are outlining happy-path scenarios only.

![](/img/diagrams/connection_overview.svg)

## Accept loop in more detail

![](/img/diagrams/connection_accept.svg)

Most of code implementation resides in [`actix-service`][service] crate. 

## Worker loop in more detail

![](/img/diagrams/connection_worker.svg)

Most of code implementation resides in [`actix-service`][service] crate. 

## Request loop roughly

![](/img/diagrams/connection_request.svg)

Most of code implementation for request loop resides in [`actix-web`][web] and [`actix-http`][http] crates. 


[service]: https://crates.io/crates/actix-service
[web]: https://crates.io/crates/actix-web
[http]: https://crates.io/crates/actix-http