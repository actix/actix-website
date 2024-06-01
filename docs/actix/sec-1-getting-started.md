---
title: Getting Started
slug: /actix/getting-started
---

import RenderCodeBlock from '@theme/CodeBlock';
import vars from "@site/vars";

# Getting Started

Let’s create and run our first actix application. We’ll create a new Cargo project that depends on actix and then run the application.

In previous section we already installed required rust version. Now let's create new cargo projects.

## Ping actor

Let’s write our first actix application! Start by creating a new binary-based Cargo project and changing into the new directory:

```bash
cargo new actor-ping
cd actor-ping
```

Now, add actix as a dependency of your project by ensuring your Cargo.toml contains the following:

<RenderCodeBlock className="language-toml">
{`[dependencies]
actix = "${vars.actorsMajorVersion}"
`}
</RenderCodeBlock>

Let's create an actor that will accept a `Ping` message and respond with the number of pings processed.

An actor is a type that implements the `Actor` trait:

```rust
use actix::prelude::*;

struct MyActor {
    count: usize,
}

impl Actor for MyActor {
    type Context = Context<Self>;
}
```

Each actor has an execution context, for `MyActor` we are going to use `Context<A>`. More information on actor contexts is available in the next section.

Now we need to define the `Message` that the actor needs to accept. The message can be any type that implements the `Message` trait.

```rust
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "usize")]
struct Ping(usize);
```

The main purpose of the `Message` trait is to define a result type. The `Ping` message defines `usize`, which indicates that any actor that can accept a `Ping` message needs to return `usize` value.

And finally, we need to declare that our actor `MyActor` can accept `Ping` and handle it. To do this, the actor needs to implement the `Handler<Ping>` trait.

```rust
impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;

        self.count
    }
}
```

That's it. Now we just need to start our actor and send a message to it. The start procedure depends on the actor's context implementation. In our case we can use `Context<A>` which is tokio/future based. We can start it with `Actor::start()` or `Actor::create()`. The first is used when the actor instance can be created immediately. The second method is used in case we need access to the context object before we can create the actor instance. In case of the `MyActor` actor we can use `start()`.

All communication with actors goes through an address. You can `do_send` a message without waiting for a response, or `send` to an actor with a specific message. Both `start()` and `create()` return an address object.

In the following example we are going to create a `MyActor` actor and send one message.

Here we use the `#[actix::main]` as way to start our System and drive our main Future so we can easily `.await` for the messages sent to the Actor.

```rust
#[actix::main]
async fn main() {
    // start new actor
    let addr = MyActor { count: 10 }.start();

    // send message and get future for result
    let res = addr.send(Ping(10)).await;

    // handle() returns tokio handle
    println!("RESULT: {}", res.unwrap() == 20);

    // stop system and exit
    System::current().stop();
}
```

`#[actix::main]` starts the system and block until future resolves.

The Ping example is available in the [examples directory](https://github.com/actix/actix/tree/master/actix/examples/).
