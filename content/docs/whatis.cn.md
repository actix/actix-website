---
title: 何为Actix 
menu: docs_intro
weight: 100
---

# Actix 指多种事情

Actix是一个强大的Rust的actor系统, 在它之上是actix-web框架。这是你在工作中大多使用的东西。Actix-web给你提供了一个有趣且快速的Web开发框架。

我们称actix-web为小而务实的框架。对于所有的意图和目的来说，这是一个有少许曲折的微框架。如果你已经是一个Rust程序员，你可能会很快熟悉它，但即使你是来自另一种编程语言，你应该会发现actix-web很容易上手。

使用actix-web开发的应用程序将在本机可执行文件中包含HTTP服务器。你可以把它放在另一个像nginx这样的HTTP服务器上。即使完全不存在另一个HTTP服务器的情况下，actix-web也足以提供HTTP 1和HTTP 2支持以及SSL/TLS。这对于构建小型服务分发非常有用。

最重要的是：actix-web可以稳定发布。
