---
title: HTTP Server Initialization
---

import CodeBlock from "@site/src/components/code_block";
import MermaidDiagram from "@site/src/components/mermaid_diagram";
import http_server from '!!raw-loader!@site/static/img/diagrams/http_server.mmd';

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

<MermaidDiagram value={http_server}  />
