---
title: HTTP/2
---

import RenderCodeBlock from '@theme/CodeBlock';
import CodeBlock from '@site/src/components/code_block';

`actix-web` automatically upgrades connections to _HTTP/2_ if possible.

# HTTP/2

## TLS Negotiation Using ALPN

When either of the `rustls-0_23` or `openssl` features are enabled, `HttpServer` provides the [`bind_rustls_0_23()`][bindrustls] and [`bind_openssl()`][bindopenssl] methods, respectively. See [TLS / HTTPS](server.md#tls--https) for certificate setup. The example below uses rustls, the recommended option.

<!-- DEPENDENCY -->

<CodeBlock example="http2" file="manifest" section="deps" language="toml"></CodeBlock>

<CodeBlock example="http2" file="main.rs" section="main" />

> Check out [the TLS examples][examples] for concrete example.

## Cleartext HTTP/2 (H2C)

When the `http2` feature is enabled, `HttpServer` provides the [`bind_auto_h2c()`][bindh2c] and [`listen_auto_h2c()`][listenh2c] methods for accepting HTTP/1.x and HTTP/2 connections without TLS on the same port. These methods automatically select the protocol by reading the first few bytes of the connection.

Clients can use HTTP/2 with prior knowledge ([RFC 7540 §3.4][rfcsection34]). Upgrades from HTTP/1.1 to HTTP/2 described in [RFC 7540 §3.2][rfcsection32] are not supported.

[rfcsection32]: https://httpwg.org/specs/rfc7540.html#rfc.section.3.2
[rfcsection34]: https://httpwg.org/specs/rfc7540.html#rfc.section.3.4
[bindrustls]: https://docs.rs/actix-web/4/actix_web/struct.HttpServer.html#method.bind_rustls_0_23
[bindopenssl]: https://docs.rs/actix-web/4/actix_web/struct.HttpServer.html#method.bind_openssl
[bindh2c]: https://docs.rs/actix-web/4/actix_web/struct.HttpServer.html#method.bind_auto_h2c
[listenh2c]: https://docs.rs/actix-web/4/actix_web/struct.HttpServer.html#method.listen_auto_h2c
[tlsalpn]: https://tools.ietf.org/html/rfc7301
[examples]: https://github.com/actix/examples/tree/master/https-tls
