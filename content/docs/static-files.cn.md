---
title: 静态文件
menu: docs_advanced
weight: 230
---

# 单文件

可以使用自定义路径模式和NamedFile来提供静态文件服务。要匹配路径尾部，我们可以使用[.*]正则表达式。

```rust
use std::path::PathBuf;
use actix_web::{App, HttpRequest, Result, http::Method, fs::NamedFile};

fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("tail")?;
    Ok(NamedFile::open(path)?)
}

fn main() {
    App::new()
        .resource(r"/a/{tail:.*}", |r| r.method(Method::GET).f(index))
        .finish();
}
```

# 目录

StaticFiles可以用作特定目录和子目录文件服务。 StaticFiles必须注册一个App::handler()方法，否则它将无法服务子路径。

```rust
use actix_web::{App, fs};

fn main() {
    App::new()
        .handler(
            "/static",
            fs::StaticFiles::new(".")
                .show_files_listing())
        .finish();
}
```

该参数是根目录。默认情况下，子目录的文件列表被禁用。尝试加载目录列表将返回404 Not Found响应。要启用文件列表，请使用[* StaticFiles :: show_files_listing（）*](https://actix.rs/actix-web/actix_web/fs/struct.StaticFiles.html#method.show_files_listing) 方法。

与其显示目录的文件列表，宁一种方法是重定向到特定的index文件。使用[*StaticFiles::index_file()*](https://actix.rs/actix-web/actix_web/fs/struct.StaticFiles.html#method.index_file)方法来配置此重定向。
