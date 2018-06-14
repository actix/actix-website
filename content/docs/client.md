# Client

actix-web provides a client for making http requests.

# Getting Started

To use async I/O we need to setup a couple of things first but you don't have
to know every detail and when making a lot of requests this scales well.


We need actix, actix_web and futures crates.

```toml
[dependencies]
actix = "0.5"
actix-web = "0.6"
futures = "0.1"
```

We need to import the crates.

```rust
extern crate actix;
extern crate actix_web;
extern crate futures;
```

Use the `client` and `Future` trait. We need the `Future` trait because the request is a future.
Read [this](https://tokio.rs/docs/getting-started/futures/) if you want to learn more about futures,
but you don't have to know the details.

```rust
use actix_web::client;
use futures::Future;
```

We need an actix system to run our request.

```rust
let sys = actix::System::new("test"); // <- Create a system. The name of the system doesn't matter.
```

Make a request and print the response. This doesn't do anything immediately, it just constructs
the future. 

```rust
let req = client::get("http://www.rust-lang.org")   // <- Create request builder
    .finish().unwrap()                    // <- Finish building the request
    .send()                               // <- Send http request
    .map_err(|_| ())                      // <- Ignore error
    .and_then(|response| {                // <- Use http response
        println!("Response: {:?}", response);
        Ok(())
    });
```

Finally make the request and print the response.

```rust
sys.run_until_complete(req).unwrap(); // <- Run the request with our system.
```
