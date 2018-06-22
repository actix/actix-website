---
title: Sentry
menu: docs_patterns
weight: 1020
---

# Sentry崩溃报告

[Sentry](https://sentry.io/)是一个崩溃报告系统，它支持基于actix错误报告的failure crate。使用Sentry中间件，可以自动将服务器错误报告给Sentry。


# Sentry中间件

该中间件捕获服务器错误范围（500-599）中的任何错误，并通过其堆栈跟踪将附加的错误发送给哨兵。

要使用中间件，需要初始化和配置Sentry，并且需要添加sentry-actix中间件。此外，注册panic处理程序以通知困难panic也是有意义的。

```rust
extern crate sentry;
extern crate sentry_actix;

use sentry_actix::SentryMiddleware;

use std::env;

fn main() {
    sentry::init("SENTRY_DSN_GOES_HERE");
    env::set_var("RUST_BACKTRACE", "1");
    sentry::integrations::panic::register_panic_handler();

    let mut app = App::with_state(state)
        .middleware(SentryMiddleware::new())
        // ...
}
```

# Reusing the Hub

如果使用这种集成，默认的sentry hub（Hub::current()）通常是错误的。要获得特定的请求，您需要使用ActixWebHubExt trait：

```rust
use sentry::{Hub, Level};
use sentry_actix::ActixWebHubExt;

let hub = Hub::from_request(req);
hub.capture_message("Something is not well", Level::Warning);
```

The hub can also be made current for the duration of a call. Then Hub::current() works correctly until the end of the run block.



```rust
use sentry::{Hub, Level};
use sentry_actix::ActixWebHubExt;

let hub = Hub::from_request(req);
Hub::run(hub, || {
    sentry::capture_message("Something is not well", Level::Warning);
});
```