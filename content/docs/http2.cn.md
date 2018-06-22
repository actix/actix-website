---
title: HTTP/2.0
menu: docs_proto
weight: 250
---

如果可能`actix-web`自动升级到*HTTP/2.0*的连接。

# 协议

*HTTP/2.0* protocol over tls  without prior knowledge requires [tls alpn](https://tools.ietf.org/html/rfc7301).

> 目前，只有`rust-openssl`支持

`alpn`协议需要启用该功能。启用后，HttpServer提供 serve_tls方法。
[serve_tls](https://actix.rs/actix-web/actix_web/server/struct.HttpServer.html#method.serve_tls) method.

```toml
[dependencies]
actix-web = { version = "{{< actix-version "actix-web" >}}", features = ["alpn"] }
openssl = { version = "0.10", features = ["v110"] }
```

```rust
use std::fs::File;
use actix_web::*;
use openssl::ssl::{SslMethod, SslAcceptor, SslFiletype};

fn main() {
    // load ssl keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(
        || App::new()
            .resource("/index.html", |r| r.f(index)))
        .bind("127.0.0.1:8080").unwrap();
        .serve_ssl(builder).unwrap();
}
```

不支持升级到[rfc section 3.2](https://http2.github.io/http2-spec/#rfc.section.3.2) 节中描述的HTTP/2.0模式 。明文连接和tls连接都支持*HTTP/2* with prior knowledge启动,[rfc section 3.4](https://http2.github.io/http2-spec/#rfc.section.3.4)

查看具体示例[examples/tls](https://github.com/actix/examples/tree/master/tls).
