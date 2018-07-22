---
title: Static Files
menu: docs_advanced
weight: 230
---

# Individual file

It is possible to serve static files with a custom path pattern and `NamedFile`. To
match a path tail, we can use a `[.*]` regex.

```rust
extern crate actix_web;
use std::path::PathBuf;
use actix_web::{App, HttpRequest, Result, http::Method, fs::NamedFile};

fn index(req: &HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("tail")?;
    Ok(NamedFile::open(path)?)
}

fn main() {
    App::new()
        .resource(r"/a/{tail:.*}", |r| r.method(Method::GET).f(index))
        .finish();
}
```

# Directory

To serve files from specific directories and sub-directories, `StaticFiles` can be used.
`StaticFiles` must be registered with an `App::handler()` method, otherwise
it will be unable to serve sub-paths.

```rust
extern crate actix_web;
use actix_web::{App, fs};

fn main() {
    App::new()
        .handler(
            "/static",
            fs::StaticFiles::new(".")
                .unwrap()
                .show_files_listing())
        .finish();
}
```

The parameter is the base directory. By default files listing for sub-directories
is disabled. Attempt to load directory listing will return *404 Not Found* response.
To enable files listing, use
[*StaticFiles::show_files_listing()*](../../actix-web/actix_web/fs/struct.StaticFiles.html#method.show_files_listing)
method.

Instead of showing files listing for directory, it is possible to redirect
to a specific index file. Use the
[*StaticFiles::index_file()*](../../actix-web/actix_web/fs/struct.StaticFiles.html#method.index_file)
method to configure this redirect.

# Configuration

Generic trait `StaticFileConfig` can be used to specify various options
for serving files:

- `content_disposition_map` - function to be used for mapping file's mime to corresponding `Content-Disposition` type
- `is_use_etag` - specifies whether `ETag` shall be calculated and included in headers.
- `is_use_last_modifier` - specifies whether file modified timestamp should be used and added to `Last-Modified` header.
- `is_method_allowed` - allows to control which HTTP methods are allowed to be used when accessing file.

All of the above methods are optional and provided with the best defaults.
But it is possible to customize any of them by implementing the trait onto own struct.

```rust
extern crate mime;
extern crate actix_web;
use actix_web::{App, HttpRequest, Result, http::Method};
use actix_web::fs::{StaticFileConfig, NamedFile};
use actix_web::http::header::DispositionType;

use std::path::PathBuf;

#[derive(Default)]
struct MyConfig;

impl StaticFileConfig for MyConfig {
    fn content_disposition_map(typ: mime::Name) -> DispositionType {
        DispositionType::Attachment
    }
}

fn index(req: &HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("tail")?;
    Ok(NamedFile::open_with_config(path, MyConfig)?)
}

fn main() {
    App::new()
        .resource(r"/a/{tail:.*}", |r| r.method(Method::GET).f(index))
        .finish();
}
```

The Configuration cal also be applied to directory service:

```rust
extern crate actix_web;

use actix_web::{App};
use actix_web::fs::{StaticFileConfig, StaticFiles};

#[derive(Default)]
struct MyConfig;

impl StaticFileConfig for MyConfig {
    fn is_use_etag() -> bool {
        false
    }

    fn is_use_last_modifier() -> bool {
        false
    }
}

fn main() {
    App::new()
        .handler(
            "/static",
            StaticFiles::with_config(".", MyConfig).unwrap()
                .show_files_listing()
        ).finish();
}
```
