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

An old version of this page recommended using a combination of [`systemfd`] and [`listenfd`], but this has many gotchas and was difficult to integrate properly, especially when part of a broader development workflow. We consider [`watchexec`] to be sufficient for auto-reloading purposes.

If you still want to use [`systemfd`]/[`listenfd`], keep these common gotchas in mind:

- It requires code changes to accept an externally provided listener, so you need extra setup (and often an extra dependency) in your server code instead of a simple `bind`.
- When a watcher process sits between `systemfd` and your server (for example, `cargo watch` or `watchexec`), you must disable the `LISTEN_PID` check (for example, `systemfd --no-pid`). Otherwise `listenfd` ignores the passed socket and your app falls back to binding a port that is already in use.
- systemd socket activation itself is Linux-only. [`systemfd`] emulates socket passing on macOS/Linux and uses a custom protocol on Windows, so behavior differs across platforms.
- Because [`systemfd`] keeps the listening socket open while a rebuild happens, the port can look "open" even when your app is not running yet; requests may hang during slow or failed rebuilds.

[`watchexec`]: https://github.com/watchexec/watchexec
[`listenfd`]: https://github.com/mitsuhiko/listenfd
[`systemfd`]: https://github.com/mitsuhiko/systemfd
