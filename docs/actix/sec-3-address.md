---
title: Address
slug: /actix/address
---

# Address

Actors communicate exclusively by exchanging messages. The sending actor can optionally wait for the response. Actors cannot be referenced directly, only by their addresses.

There are several ways to get the address of an actor. The `Actor` trait provides two helper methods for starting an actor. Both return the address of the started actor.

Here is an example of `Actor::start()` method usage. In this example `MyActor` actor is asynchronous and is started in the same thread as the caller - threads are covered in the [SyncArbiter] chapter.

```rust
struct MyActor;
impl Actor for MyActor {
    type Context = Context<Self>;
}

let addr = MyActor.start();
```

An async actor can get its address from the `Context` struct. The context needs to implement the `AsyncContext` trait. `AsyncContext::address()` provides the actor's address.

```rust
struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
       let addr = ctx.address();
    }
}
```

[SyncArbiter]: ./sync-arbiter

## Message

To be able to handle a specific message the actor has to provide a [`Handler<M>`] implementation for this message. All messages are statically typed. The message can be handled in an asynchronous fashion. The actor can spawn other actors or add futures or streams to the execution context. The actor trait provides several methods that allow controlling the actor's lifecycle.

To send a message to an actor, the `Addr` object needs to be used. `Addr` provides several ways to send a message.

- `Addr::do_send(M)` - this method ignores any errors in message sending. If the mailbox is full the message is still queued, bypassing the limit. If the actor's mailbox is closed, the message is silently dropped. This method does not return the result, so if the mailbox is closed and a failure occurs, you won't have an indication of this.

- `Addr::try_send(M)` - this method tries to send the message immediately. If the mailbox is full or closed (actor is dead), this method returns a [`SendError`].

- `Addr::send(M)` - This message returns a future object that resolves to a result of a message handling process. If the returned `Future` object is dropped, the message is cancelled.

[`Handler<M>`]: https://docs.rs/actix/latest/actix/trait.Handler.html
[`SendError`]: https://docs.rs/actix/latest/actix/prelude/enum.SendError.html

## Recipient

Recipient is a specialized version of an address that supports only one type of message. It can be used in case the message needs to be sent to a different type of actor. A recipient object can be created from an address with `Addr::recipient()`.

Address objects require an actor type, but if we just want to send a specific message to an actor that can handle the message, we can use the Recipient interface.

For example recipient can be used for a subscription system. In the following example `OrderEvents` actor sends a `OrderShipped` message to all subscribers. A subscriber can be any actor that implements the `Handler<OrderShipped>` trait.

```rust
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
struct OrderShipped(usize);

#[derive(Message)]
#[rtype(result = "()")]
struct Ship(usize);

/// Subscribe to order shipped event.
#[derive(Message)]
#[rtype(result = "()")]
struct Subscribe(pub Recipient<OrderShipped>);

/// Actor that provides order shipped event subscriptions
struct OrderEvents {
    subscribers: Vec<Recipient<OrderShipped>>,
}

impl OrderEvents {
    fn new() -> Self {
        OrderEvents {
            subscribers: vec![]
        }
    }
}

impl Actor for OrderEvents {
    type Context = Context<Self>;
}

impl OrderEvents {
    /// Send event to all subscribers
    fn notify(&mut self, order_id: usize) {
        for subscr in &self.subscribers {
           subscr.do_send(OrderShipped(order_id));
        }
    }
}

/// Subscribe to shipment event
impl Handler<Subscribe> for OrderEvents {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, _: &mut Self::Context) {
        self.subscribers.push(msg.0);
    }
}

/// Subscribe to ship message
impl Handler<Ship> for OrderEvents {
    type Result = ();
    fn handle(&mut self, msg: Ship, ctx: &mut Self::Context) -> Self::Result {
        self.notify(msg.0);
        System::current().stop();
    }
}

/// Email Subscriber
struct EmailSubscriber;
impl Actor for EmailSubscriber {
    type Context = Context<Self>;
}

impl Handler<OrderShipped> for EmailSubscriber {
    type Result = ();
    fn handle(&mut self, msg: OrderShipped, _ctx: &mut Self::Context) -> Self::Result {
        println!("Email sent for order {}", msg.0)
    }

}
struct SmsSubscriber;
impl Actor for SmsSubscriber {
    type Context = Context<Self>;
}

impl Handler<OrderShipped> for SmsSubscriber {
    type Result = ();
    fn handle(&mut self, msg: OrderShipped, _ctx: &mut Self::Context) -> Self::Result {
        println!("SMS sent for order {}", msg.0)
    }

}

#[actix::main]
async fn main() -> Result<(), actix::MailboxError> {
    let email_subscriber = Subscribe(EmailSubscriber {}.start().recipient());
    let sms_subscriber = Subscribe(SmsSubscriber {}.start().recipient());
    let order_event = OrderEvents::new().start();
    order_event.send(email_subscriber).await?;
    order_event.send(sms_subscriber).await?;
    order_event.send(Ship(1)).await?;
    Ok(())
}
```
