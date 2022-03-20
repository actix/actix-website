---
title: HTTP/2
menu: docs_protocols
weight: 250
---

Actix Web will prefer HTTP/2 connections if the client signals support for it via [TLS ALPN][tlsalpn].

# Negotiation

<!-- TODO: use rustls example -->

When either of the `rustls` or `openssl` features are enabled, `HttpServer` provides the [bind_rustls][bindrustls] method and [bind_openssl][bindopenssl] methods, respectively.

```toml
[dependencies]
actix-web = { version = "{{< actix-version "actix-web" >}}", features = ["openssl"] }
openssl = { version = "0.10", features = ["v110"] }
```

{{< include-example example="http2" file="main.rs" section="main" >}}

Upgrades to HTTP/2 described in [RFC 7540 §3.2][rfcsection32] are not supported. Starting HTTP/2 with prior knowledge is supported for both cleartext and TLS connections ([RFC 7540 §3.4][rfcsection34]) (when using the lower level `actix-http` service builders).

> Check out [the TLS examples][examples] for concrete example.

[rfcsection32]: https://httpwg.org/specs/rfc7540.html#rfc.section.3.2
[rfcsection34]: https://httpwg.org/specs/rfc7540.html#rfc.section.3.4
[bindrustls]: https://docs.rs/actix-web/4/actix_web/struct.HttpServer.html#method.bind_rustls
[bindopenssl]: https://docs.rs/actix-web/4/actix_web/struct.HttpServer.html#method.bind_openssl
[tlsalpn]: https://tools.ietf.org/html/rfc7301
[examples]: https://github.com/actix/examples/tree/master/https-tls
