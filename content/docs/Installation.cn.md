---
title: 安装
menu: docs_intro
weight: 110
---

# 安装 Rust

既然actix-web是一个Rust框架，你需要Rust来使用它。如果您还没有它，我们建议您使用rustup来管理您的Rust安装。该[官员Rust指南](https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html) 有精彩的入门部分。

我们目前至少需要Rust 1.24，因此请确保您运行的rustup update 是最新且最好的Rust版本。特别是本指南将假定您实际运行Rust 1.26或更高版本。


# 安装 `actix-web`

感谢Rust的cargo包管理，您不需要明确安装 actix-web。只需要声明依赖就行了。对于想要使用actix-web的开发版本的情况，您可以直接依赖git存储库。

Release版本：

```ini
[dependencies]
actix-web = "{{< actix-version "actix-web" >}}"
```

Development 版本：

```ini
[dependencies]
actix-web = { git = "https://github.com/actix/actix-web" }
```

# 深入

在这里有两条你可以选择的路径。你可以沿着指南或者如果你非常不耐烦，你可能想看看我们 广泛的[示例库](https://github.com/actix/examples)并运行包含的示例。这里举例说明如何运行包含的basics 示例：

```
git clone https://github.com/actix/examples
cd examples/basics
cargo run
```
