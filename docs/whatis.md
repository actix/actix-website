---
title: What is Actix Web
---

import vars from "@site/vars";

# Actix Web is part of an Ecosystem of Crates

Long ago, Actix Web was built on top of the `actix` actor framework. Now, Actix Web is largely unrelated to the actor framework and is built using a different system. Though `actix` is still maintained, its usefulness as a general tool is diminishing as the futures and async/await ecosystem matures. At this time, the use of `actix` is only required for WebSocket endpoints.

We call Actix Web a powerful and pragmatic framework. For all intents and purposes it's a micro-framework with a few twists. If you are already a Rust programmer you will probably find yourself at home quickly, but even if you are coming from another programming language you should find Actix Web easy to pick up.

<!-- TODO -->
<!-- actix-extras -->

An application developed with Actix Web will expose an HTTP server contained within a native executable. You can either put this behind another HTTP server like nginx or serve it up as-is. Even in the complete absence of another HTTP server, Actix Web is powerful enough to provide HTTP/1 and HTTP/2 support as well as TLS (HTTPS). This makes it useful for building small services ready for production.

<p>
Most importantly: Actix Web runs on Rust { vars.rustVersion } or later and it works with stable releases.
</p>

<!-- TODO -->
<!-- which is built upon the fantastic [Tokio][tokio] asynchronous I/O system -->

<!-- LINKS -->

[tokio]: https://tokio.rs
