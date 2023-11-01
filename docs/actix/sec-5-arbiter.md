---
title: Arbiter
slug: /actix/arbiter
---

# Arbiter

`Arbiter`s provide an asynchronous execution context for `Actor`s, `functions` and `futures`. Where an actor contains a `Context` that defines its Actor specific execution state, Arbiters host the environment where an actor runs.

As a result Arbiters perform a number of functions. Most notably, they are able to spawn a new OS thread, run an event loop, spawn tasks asynchronously on that event loop, and act as helpers for asynchronous tasks.

## System and Arbiter

In all our previous code examples the function `System::new` creates an Arbiter for your actors to run inside. When you call `start()` on your actor it is then running inside of the System Arbiter's thread. In many cases, this is all you will need for a program using Actix.

While it only uses one thread, it uses the very efficient event loop pattern which works well for asynchronous events. To handle synchronous, CPU-bound tasks, it's better to avoid blocking the event loop and instead offload the computation to other threads. For this usecase, read the next section and consider using [`SyncArbiter`](./sync-arbiter).

## The event loop

One `Arbiter` is in control of one thread with one event pool. When an Arbiter spawns a task (via `Arbiter::spawn`, `Context<Actor>::run_later`, or similar constructs), the Arbiter queues the task for execution on that task queue. When you think `Arbiter`, you can think "single-threaded event loop".

Actix in general does support concurrency, but normal `Arbiter`s (not `SyncArbiter`s) do not. To use Actix in a concurrent way, you can spin up multiple `Arbiter`s using `Arbiter::new`, `ArbiterBuilder`, or `Arbiter::start`.

When you create a new Arbiter, this creates a new execution context for Actors. The new thread is available to add new Actors to it, but Actors cannot freely move between Arbiters: they are tied to the Arbiter they were spawned in. However, Actors on different Arbiters can still communicate with each other using the normal `Addr`/`Recipient` methods. The method of passing messages is agnostic to whether the Actors are running on the same or different Arbiters.

## Using Arbiter for resolving async events

If you aren't an expert in Rust Futures, Arbiter can be a helpful and simple wrapper to resolving async events in order. Consider we have two actors, A and B, and we want to run an event on B only once a result from A is completed. We can use `Arbiter::spawn` to assist with this task.

```rust
use actix::prelude::*;

struct SumActor {}

impl Actor for SumActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "usize")]
struct Value(usize, usize);

impl Handler<Value> for SumActor {
    type Result = usize;

    fn handle(&mut self, msg: Value, _ctx: &mut Context<Self>) -> Self::Result {
        msg.0 + msg.1
    }
}

struct DisplayActor {}

impl Actor for DisplayActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
struct Display(usize);

impl Handler<Display> for DisplayActor {
    type Result = ();

    fn handle(&mut self, msg: Display, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Got {:?}", msg.0);
    }
}

fn main() {
    let system = System::new("single-arbiter-example");

    // Define an execution flow using futures
    let execution = async {
        // `Actor::start` spawns the `Actor` on the *current* `Arbiter`, which
        // in this case is the System arbiter
        let sum_addr = SumActor {}.start();
        let dis_addr = DisplayActor {}.start();

        // Start by sending a `Value(6, 7)` to our `SumActor`.
        // `Addr::send` responds with a `Request`, which implements `Future`.
        // When awaited, it will resolve to a `Result<usize, MailboxError>`.
        let sum_result = sum_addr.send(Value(6, 7)).await;

        match sum_result {
            Ok(res) => {
                // `res` is now the `usize` returned from `SumActor` as a response to `Value(6, 7)`
                // Once the future is complete, send the successful response (`usize`)
                // to the `DisplayActor` wrapped in a `Display
                dis_addr.send(Display(res)).await;
            }
            Err(e) => {
                eprintln!("Encountered mailbox error: {:?}", e);
            }
        };
    };

    // Spawn the future onto the current Arbiter/event loop
    Arbiter::current().spawn(execution);

    // We only want to do one computation in this example, so we
    // shut down the `System` which will stop any Arbiters within
    // it (including the System Arbiter), which will in turn stop
    // any Actor Contexts running within those Arbiters, finally
    // shutting down all Actors.
    System::current().stop();

    system.run();
}
```
