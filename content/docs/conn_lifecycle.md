---
title: Connection Lifecycle
menu: docs_architecture
weight: 20
---


# Architecture overview

After Server has started with all connections listening, there are two main loops responsible for processing incoming client connections on established sockets Accept and Worker. Application level protocol processing happens in separate loop spawned from Worker after connection accepted.

![](/img/diagrams/connection_overview.svg)

## Accept loop in more detail

![](/img/diagrams/connection_accept.svg)

Most of code implementation resides in [actix-service](https://crates.io/crates/actix-service) crate. 

## Worker loop in more detail

![](/img/diagrams/connection_worker.svg)

Most of code implementation resides in [actix-service](https://crates.io/crates/actix-service) crate. 

## Request loop roughly

![](/img/diagrams/connection_request.svg)

Most of code implementation for request loop resides in [actix-web](https://crates.io/crates/actix-web) and [actix-http](https://crates.io/crates/actix-http) crates. 