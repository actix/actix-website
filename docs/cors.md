---
title: CORS
---

# CORS

[Cross-Origin Resource Sharing (CORS)][mdn-cors] is usually configured once at server startup, but
many Actix Web services need to load it from YAML, TOML, or CLI flags. The main thing to know is
that [`actix-cors`][actix-cors] has separate APIs for "allow anything" and "allow these exact
values".

In particular, this does **not** work:

```rust
use actix_cors::Cors;

let cors = Cors::default().allowed_origin("*");
```

`allowed_origin("*")` is rejected during startup. Use `allow_any_origin()` for wildcard behavior.

## Basic Examples

### Allow One Frontend Origin

This is the most common production setup for a browser app talking to one API:

```rust
use actix_cors::Cors;
use actix_web::{http::header, App};

let cors = Cors::default()
    .allowed_origin("https://app.example.com")
    .allowed_methods(vec!["GET", "POST"])
    .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
    .max_age(3600);

let app = App::new().wrap(cors);
```

### Allow Several Frontend Origins

Add each allowed origin separately:

```rust
use actix_cors::Cors;
use actix_web::App;

let cors = Cors::default()
    .allowed_origin("https://app.example.com")
    .allowed_origin("https://admin.example.com")
    .allowed_methods(vec!["GET", "POST", "DELETE"]);

let app = App::new().wrap(cors);
```

### Allow Any Origin For a Public API

If the API is public and does not need cookies or other credentials, allow any origin:

```rust
use actix_cors::Cors;
use actix_web::App;

let cors = Cors::default()
    .allow_any_origin()
    .send_wildcard()
    .allow_any_method()
    .allow_any_header();

let app = App::new().wrap(cors);
```

### Allow Credentials

If the browser must send cookies or authenticated cross-origin requests, use an explicit origin
list instead of `*`:

```rust
use actix_cors::Cors;
use actix_web::{http::header, App};

let cors = Cors::default()
    .allowed_origin("https://app.example.com")
    .supports_credentials()
    .allowed_methods(vec!["GET", "POST"])
    .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE]);

let app = App::new().wrap(cors);
```

### Apply CORS To Only One Scope

You can wrap a scope instead of the whole application:

```rust
use actix_cors::Cors;
use actix_web::{web, App};

let cors = Cors::default()
    .allowed_origin("https://app.example.com")
    .allowed_methods(vec!["GET", "POST"]);

let app = App::new().service(
    web::scope("/api")
        .wrap(cors)
        .route("/health", web::get().to(|| async { "ok" })),
);
```

## Recommended Config Shape

This shape maps cleanly to the `actix-cors` builder:

```yaml
cors:
  origins: "*"
  methods: [GET, POST, OPTIONS]
  headers: [AUTHORIZATION, ACCEPT, CONTENT-TYPE]
  expose-headers: [X-REQUEST-ID]
  credentials: false
  send-wildcard: true
  max-age: 3600
  block-on-origin-mismatch: false
```

For tighter production settings, prefer an explicit allowlist:

```yaml
cors:
  origins:
    - https://app.example.com
    - https://admin.example.com
  methods: [GET, POST]
  headers: [AUTHORIZATION, CONTENT-TYPE]
  credentials: true
  max-age: 3600
```

## Translating Config Into `Cors`

The simplest approach is to normalize your config into either a single string or a list of strings,
then handle `"*"` explicitly:

```rust
use actix_cors::Cors;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum OneOrMany {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Clone, Deserialize)]
struct CorsSettings {
    origins: Option<OneOrMany>,
    methods: Option<OneOrMany>,
    headers: Option<OneOrMany>,
    #[serde(rename = "expose-headers")]
    expose_headers: Option<OneOrMany>,
    credentials: bool,
    #[serde(rename = "send-wildcard")]
    send_wildcard: bool,
    #[serde(rename = "max-age")]
    max_age: Option<usize>,
    #[serde(rename = "block-on-origin-mismatch")]
    block_on_origin_mismatch: bool,
}

fn cors_from_settings(settings: &CorsSettings) -> Cors {
    let mut cors = Cors::default();

    match settings.origins.as_ref() {
        None => {}
        Some(OneOrMany::One(origin)) if origin == "*" => {
            cors = cors.allow_any_origin();
        }
        Some(OneOrMany::One(origin)) => {
            cors = cors.allowed_origin(origin);
        }
        Some(OneOrMany::Many(origins)) => {
            for origin in origins {
                cors = cors.allowed_origin(origin);
            }
        }
    }

    match settings.methods.as_ref() {
        None => {}
        Some(OneOrMany::One(method)) if method == "*" => {
            cors = cors.allow_any_method();
        }
        Some(OneOrMany::One(method)) => {
            cors = cors.allowed_methods([method.as_str()]);
        }
        Some(OneOrMany::Many(methods)) => {
            cors = cors.allowed_methods(methods.iter().map(String::as_str));
        }
    }

    match settings.headers.as_ref() {
        None => {}
        Some(OneOrMany::One(header)) if header == "*" => {
            cors = cors.allow_any_header();
        }
        Some(OneOrMany::One(header)) => {
            cors = cors.allowed_header(header.as_str());
        }
        Some(OneOrMany::Many(headers)) => {
            cors = cors.allowed_headers(headers.iter().map(String::as_str));
        }
    }

    match settings.expose_headers.as_ref() {
        None => {}
        Some(OneOrMany::One(header)) if header == "*" => {
            cors = cors.expose_any_header();
        }
        Some(OneOrMany::One(header)) => {
            cors = cors.expose_headers([header.as_str()]);
        }
        Some(OneOrMany::Many(headers)) => {
            cors = cors.expose_headers(headers.iter().map(String::as_str));
        }
    }

    if settings.credentials {
        cors = cors.supports_credentials();
    }

    if settings.send_wildcard {
        cors = cors.send_wildcard();
    }

    if let Some(max_age) = settings.max_age {
        cors = cors.max_age(max_age);
    }

    cors.block_on_origin_mismatch(settings.block_on_origin_mismatch)
}
```

## Mapping Rules

- `origins: "*"` maps to `allow_any_origin()`, not `allowed_origin("*")`.
- `methods: "*"` maps to `allow_any_method()`.
- `headers: "*"` maps to `allow_any_header()`.
- A single origin like `https://app.example.com` maps to one `allowed_origin(...)` call.
- A list of origins maps to repeated `allowed_origin(...)` calls.
- A single method or header can be passed as a one-item iterator, or handled with the singular
  builder methods.

## Wildcards, Credentials, and Caches

`allow_any_origin()` and `send_wildcard()` are different:

- `allow_any_origin()` accepts any origin.
- `send_wildcard()` changes the response header from echoing the request origin to sending
  `Access-Control-Allow-Origin: *`.

That distinction matters because credentials and wildcard responses cannot be combined:

```rust
use actix_cors::Cors;

let cors = Cors::default()
    .allow_any_origin()
    .supports_credentials()
    .send_wildcard();
```

That configuration fails during startup. If your browser clients need cookies or authorization
headers, prefer an explicit origin allowlist instead of `*`.

`actix-cors` also enables the `Vary` header by default. Keep that default unless you fully
understand the caching implications. It tells CDNs and proxies that the CORS response can change
based on request headers.

## When To Use `allowed_origin_fn`

Most applications should keep CORS static and startup-configured. Use
[`allowed_origin_fn`][allowed-origin-fn] only when your allowlist really must depend on request
data or pattern matching, such as tenant subdomains:

```rust
use actix_cors::Cors;

let cors = Cors::default().allowed_origin_fn(|origin, _req_head| {
    origin.as_bytes().ends_with(b".example.com")
});
```

That is different from loading a normal allowlist from config. If your configuration is just
"one value, many values, or `*`", prefer the static builder methods shown above.

## Applying CORS To Your App

Once the builder has been created from config, wrap it like any other middleware:

```ignore
use actix_cors::Cors;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = load_settings();

    HttpServer::new(move || {
        App::new()
            .wrap(cors_from_settings(&settings.cors))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

See the [`Cors` API docs][cors-api] for the full builder surface.

[mdn-cors]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/CORS
[actix-cors]: https://docs.rs/actix-cors/latest/actix_cors/
[cors-api]: https://docs.rs/actix-cors/latest/actix_cors/struct.Cors.html
[allowed-origin-fn]: https://docs.rs/actix-cors/latest/actix_cors/struct.Cors.html#method.allowed_origin_fn
