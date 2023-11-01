---
title: SyncArbiter
slug: /actix/sync-arbiter
---

# SyncArbiter

When you normally run Actors, there are multiple Actors running on the System's Arbiter thread, using its event loop. However for CPU bound workloads, or highly concurrent workloads, you may wish to have an Actor running multiple instances in parallel.

This is what a SyncArbiter provides - the ability to launch multiple instances of an Actor on a pool of OS threads.

It's important to note a SyncArbiter can only host a single type of Actor. This means you need to create a SyncArbiter for each type of Actor you want to run in this manner.

## Creating a Sync Actor

When implementing your Actor to be run on a SyncArbiter, it requires that your Actor's Context is changed from `Context` to `SyncContext`.

```rust
use actix::prelude::*;

struct MySyncActor;

impl Actor for MySyncActor {
    type Context = SyncContext<Self>;
}
```

## Starting the Sync Arbiter

Now that we have defined a Sync Actor, we can run it on a thread pool, created by our `SyncArbiter`. We can only control the number of threads at SyncArbiter creation time - we can't add/remove threads later.

```rust
use actix::prelude::*;

struct MySyncActor;

impl Actor for MySyncActor {
    type Context = SyncContext<Self>;
}

let addr = SyncArbiter::start(2, || MySyncActor);
```

We can communicate with the addr the same way as we have with our previous Actors that we started. We can send messages, receive futures and results, and more.

## Sync Actor Mailboxes

Sync Actors have no Mailbox limits, but you should still use `do_send`, `try_send` and `send` as normal to account for other possible errors or sync vs async behavior.
