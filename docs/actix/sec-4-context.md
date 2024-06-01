---
title: Context
slug: /actix/context
---

# Context

Actors all maintain an internal execution context, or state. This allows an actor to determine its own Address, change mailbox limits, or stop its execution.

## Mailbox

All messages go to the actor's mailbox first, then the actor's execution context calls specific message handlers. Mailboxes in general are bounded. The capacity is specific to the context implementation. For the `Context` type the capacity is set to 16 messages by default and can be increased with [`Context::set_mailbox_capacity()`].

```rust
struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.set_mailbox_capacity(1);
    }
}

let addr = MyActor.start();
```

Remember that this doesn't apply to `Addr::do_send(M)` which bypasses the Mailbox queue limit, or `AsyncContext::notify(M)` and `AsyncContext::notify_later(M, Duration)` which bypasses the mailbox entirely.

[`Context::set_mailbox_capacity()`]: https://docs.rs/actix/latest/actix/struct.Context.html#method.set_mailbox_capacity

## Getting your actors Address

An actor can view its own address from its context. Perhaps you want to requeue an event for later, or you want to transform the message type. Maybe you want to respond with your address to a message. If you want an actor to send a message to itself, have a look at `AsyncContext::notify(M)` instead.

To get your address from the context you call [`Context::address()`]. An example is:

```rust
struct MyActor;

struct WhoAmI;

impl Message for WhoAmI {
    type Result = Result<actix::Addr<MyActor>, ()>;
}

impl Actor for MyActor {
    type Context = Context<Self>;
}

impl Handler<WhoAmI> for MyActor {
    type Result = Result<actix::Addr<MyActor>, ()>;

    fn handle(&mut self, msg: WhoAmI, ctx: &mut Context<Self>) -> Self::Result {
        Ok(ctx.address())
    }
}

let who_addr = addr.do_send(WhoAmI{});
```

[`Context::address()`]: https://docs.rs/actix/latest/actix/struct.Context.html#method.address

## Stopping an Actor

From within the actors execution context you can choose to stop the actor from processing any future Mailbox messages. This could be in response to an error condition, or as part of program shutdown. To do this you call [`Context::stop()`].

This is an adjusted Ping example that stops after 4 pings are received.

```rust
impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;

        if self.count > 5 {
            println!("Shutting down ping receiver.");
            ctx.stop()
        }

        self.count
    }
}

#[actix::main]
async fn main() {
    // start new actor
    let addr = MyActor { count: 10 }.start();

    // send message and get future for result
    let addr_2 = addr.clone();
    let res = addr.send(Ping(6)).await;

    match res {
        Ok(_) => assert!(addr_2.try_send(Ping(6)).is_err()),
        _ => {}
    }
}
```

[`Context::stop()`]: https://docs.rs/actix/latest/actix/struct.Context.html#method.stop
