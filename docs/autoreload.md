---
title: Auto-Reloading
---

# Auto-Reloading Development Server

During development it can be very handy to have cargo automatically recompile the code on changes. This can be accomplished very easily by using [`watchexec`].

The following command runs/restarts `cargo run` every time a file with the `rs` extension changes inside the current directory (including subdirectories):

```sh
watchexec -e rs -r cargo run
```

If you want to watch all files in a specific directory, you can use the the `-w` argument:

```sh
watchexec -w src -r cargo run
```

## Historical Note

An old version of this page recommended using a combination of systemfd and listenfd, but this has many gotchas and was difficult to integrate properly, especially when part of a broader development workflow. We consider [`watchexec`] to be sufficient for auto-reloading purposes.

[`watchexec`]: https://github.com/watchexec/watchexec
