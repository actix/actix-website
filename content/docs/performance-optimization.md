---
title: Performance Optimization
menu: docs_advanced
weight: 240
---

# Performance Optimization

Your actix system likely has unrealized performance, significant in some cases, available through optimization.

A simple iterative process will help you identify actual optimizations for your application: Benchmark program, change one thing, benchmark program, compare results, repeat.

The following program changes have been known to optimize the performance of actix systems in at least some cases:

## actix
* [API] Replace usage of do_send() with an async equivalent such as send().
* [API] Remove Futures chained following `send().into_actor(self)` from hot call sites as the Tokio event loop becomes overwhelmed.
* [ARCHITECTURE] Move high traffic (sending/receiving messages) actors into their own Arbiter. (thread)
* [DEBUG MODE] A panic with the message "Use Self::Context::notify() instead of direct use of address" can be generated under high load in [debug mode](https://github.com/actix/actix/blob/f4319ae1946233e7054091b281b95c6fa08081c1/src/mailbox.rs#L103).

## actix web
* [NETWORKING] As the current implementation of *nix domain sockets (UDS) is single-threaded, reimplement with an IP port. (PR for a multi-threaded UDS implementation welcome!)
