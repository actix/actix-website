---
title: Auto-Reloading
---

# Auto-Reloading Development Server

During development it can be very handy to have cargo automatically recompile the code on changes. This can be accomplished very easily by using [`cargo-watch`].

```sh
 cargo watch -x run
 ```

Warning: [`cargo-watch`] before version 8.3 has an issue related to sending signals to child processes, caused by to changes in Rust 1.66. Ensure that you are using version 8.4 or later of [`cargo-watch`].

## Historical Note

An old version of this page recommended using a combination of systemfd and listenfd, but this has many gotchas and was difficult to integrate properly, especially when part of a broader development workflow. We consider [`cargo-watch`] to be sufficient for auto-reloading purposes.

[`cargo-watch`]: https://github.com/passcod/cargo-watch
