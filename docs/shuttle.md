---
title: Hosting on Shuttle
---

import CodeBlock from '@site/src/components/code_block';

# Hosting on Shuttle

<img width="300" src="https://raw.githubusercontent.com/shuttle-hq/shuttle/master/assets/logo-rectangle-transparent.png" alt="Shuttle Logo"/>

> [**Shuttle**](https://www.shuttle.rs) is a Rust-native cloud development platform that lets you deploy your Rust apps for free.

Shuttle has out-of-the-box support for Actix Web. Follow these steps to host your web service on Shuttle:

1. Add Shuttle dependencies to `Cargo.toml`:

<CodeBlock example="shuttle" file="manifest" section="shuttle-deps" language="toml" />

2. Add the `#[shuttle_runtime::main]` annotation and update the `main` function as follows:

<CodeBlock example="shuttle" section="shuttle-hello-world" />

3. Install `cargo-shuttle`:

```sh
cargo install cargo-shuttle
```

4. Create your project on the Shuttle platform:

```sh
cargo shuttle project start
```

5. Deploy! 🚀

```sh
cargo shuttle deploy
```

You can run `cargo shuttle run` to test your application locally.

Check out some complete Actix Web examples [here](https://github.com/shuttle-hq/shuttle-examples/tree/main/actix-web).
