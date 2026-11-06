---
title: What Is Actix Web
description: >-
  Build HTTP APIs and web applications in Rust with Actix Web. Read about request handling,
  middleware, and the crates available for common web development tasks.
---

# What Is Actix Web

Actix Web is a web framework for building HTTP APIs and web applications in Rust. It provides routing, typed request data extraction, middleware, and an HTTP server. You write request handlers as async functions and use Rust types to read requests and build responses.

An Actix Web application compiles to a native executable with its own HTTP server. You can serve requests directly or put it behind a reverse proxy such as HAProxy or Caddy. Actix Web supports HTTP/1.1, HTTP/2, and TLS (HTTPS), and works with the [Tokio](https://tokio.rs) async ecosystem.

## What Can You Build With Actix Web?

- **HTTP APIs:** Read JSON, path parameters, and query strings with [extractors](extractors.md), then return typed [JSON responses](response.md#json-response).
- **Web applications:** Handle [forms](extractors.md#url-encoded-forms), serve [static files](static-files.md), and connect handlers to a [database](databases.md).
- **Real-time services:** Use [WebSockets](websockets.md) for chat, notifications, and other applications that exchange messages over a persistent connection.

## How an Application Works

An [`App`](application.md) groups routes, middleware, and application state. The [router](url-dispatch.md) matches an incoming request to a [handler](handlers.md). Extractors read the data the handler needs, and the handler returns a value that can be converted into an HTTP response.

[Middleware](middleware.md) can inspect or change requests and responses. Use it for tasks such as logging and response compression. You can apply middleware to the whole application or to a scope of routes.

[`HttpServer`](server.md) runs the application and accepts connections. It provides configuration for worker threads, TLS, keep-alive connections, and graceful shutdown. Actix Web also provides [test utilities](testing.md) for applications, handlers, and extractors.

## Ecosystem Crates {#actix-web-is-part-of-an-ecosystem-of-crates}

For features such as file uploads or sessions, add the relevant crate to your application:

- [`actix-files`](https://docs.rs/actix-files): Serve files and directories.
- [`actix-multipart`](https://docs.rs/actix-multipart): Process multipart forms and file uploads.
- [`actix-cors`](https://docs.rs/actix-cors): Configure cross-origin requests. See the [CORS guide](cors.md) for examples.
- [`actix-session`](https://docs.rs/actix-session): Manage session state across requests.
- [`actix-ws`](https://docs.rs/actix-ws): Handle WebSocket connections with async tasks.

The [community crates](/community/crates) page lists more integrations and extensions.

## Relationship to the Actor Framework

Actix Web was originally built on the `actix` actor framework, but has not depended on it since 2019. You do not need to use actors to build an Actix Web application.

## Getting Started

Actix Web runs on stable Rust. Follow the [getting started guide](getting-started.md) to install the required tools and run your first server. For complete applications, browse the [examples repository](https://github.com/actix/examples). Use the [API documentation](https://docs.rs/actix-web) when you need details about a specific type or method.
