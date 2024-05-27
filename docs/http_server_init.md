---
title: HTTP Server Initialization
---

# Architecture overview

Below is a diagram of HttpServer initialization, which happens on the following code

```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::to(|| HttpResponse::Ok()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

![HTTP Server Diagram](/img/diagrams/http_server.svg)
