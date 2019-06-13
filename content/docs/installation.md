---
title: Installation
menu: docs_intro
weight: 110
---

# Installing Rust

Since `actix-web` is a Rust framework you will need Rust to get started with it.
If you don't have it yet we recommend you use `rustup` to manage your Rust
installation.  The [official rust
guide](https://doc.rust-lang.org/book/ch01-01-installation.html)
has a wonderful section on getting started.

We currently require at least Rust {{< rust-version "actix-web" >}} so make sure you
run `rustup update` to have the latest and greatest Rust version available.  In
particular this guide will assume that you actually run Rust
{{< rust-version "actix-web" >}} or later.

# Installing `actix-web`

Thanks to Rust's `cargo` package manager you won't need to explicitly install
`actix-web`.  Just depend on it and you're ready to go.  For the unlikely
case that you want to use the development version of actix-web you can
depend on the git repository directly.

Release version:

```ini
[dependencies]
actix-web = "{{< actix-version "actix-web" >}}"
```

Development version:

```ini
[dependencies]
actix-web = { git = "https://github.com/actix/actix-web" }
```

# Diving In

There are two paths you can take here.  You can follow the guide along or if
you are very impatient you might want to have a look at our
[extensive example repository](https://github.com/actix/examples) and run the
included examples.  Here for instance is how you run the included `basics`
example:

```
git clone https://github.com/actix/examples
cd examples/basics
cargo run
```
