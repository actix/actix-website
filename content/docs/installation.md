---
title: Installation
menu: docs_intro
weight: 110
---

# Installing Rust

Since `actix-web` is a Rust framework, you will need Rust to get started. If
you don't have it yet, we recommend the use of `rustup` to manage your Rust
installation.  The [official Rust
guide](https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html)
has a wonderful section on getting started with `rustup`.

We currently require at least Rust 1.24, so make sure to run `rustup update`
to have the latest and greatest Rust version available.  In particular, this
guide will assume that you actually run Rust 1.26 or later.

# Installing `actix-web`

Thanks to Rust's `cargo` package manager, you won't need to explicitly install
`actix-web`.  Just add it to your dependencies and you're ready to go. In the
unlikely case where you want to use the development version of `actix-web`, you
can depend directly on the Git repository.

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

There are two paths you can take from here.  You can follow the guide along or,
if you are very impatient, you might want to have a look at our
[extensive example repository](https://github.com/actix/examples), and run the
included examples.  For instance, here is how you can run the included `basics`
example:

```
git clone https://github.com/actix/examples
cd examples/basics
cargo run
```
