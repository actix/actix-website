---
title: Extractors
menu: docs_basics
weight: 170
---

# Type-safe information extraction

Actix provides facility for type-safe request information extraction. By default,
actix provides several extractor implementations.

# Accessing Extractors

How you access an Extractor depends on whether you are using a handler function
or a custom Handler type.

## Within Handler Functions

An Extractor can be passed to a handler function as a function parameter
*or* accessed within the function by calling the ExtractorType::<...>::extract(req)
function.
```rust

// Option 1:  passed as a parameter to a handler function
fn index((params, info): (Path<(String, String,)>, Json<MyInfo>)) -> HttpResponse {
   ... 
}


// Option 2:  accessed by calling extract() on the Extractor

use actix_web::FromRequest;

fn index(req: HttpRequest) -> HttpResponse {
	let params = Path::<(String, String)>::extract(&req);
	let info = Json::<MyInfo>::extract(&req); 

	...
}
```

## Within Custom Handler Types

Like a handler function, a custom Handler type can *access* an Extractor by
calling the ExtractorType::<...>::extract(&req) function.  An Extractor 
*cannot* be passed as a parameter to a custom Handler type because a custom 
Handler type must follow the ``handle`` function signature specified by the 
Handler trait it implements.

```rust

struct MyHandler(String);

impl<S> Handler<S> for MyHandler {
    type Result = HttpResponse;

    /// Handle request
    fn handle(&mut self, req: HttpRequest<S>) -> Self::Result {
		let params = Path::<(String, String)>::extract(&req);
		let info = Json::<MyInfo>::extract(&req); 

		...
			
        HttpResponse::Ok().into()
    }
}

```

# Path

[*Path*](../../actix-web/actix_web/struct.Path.html) provides information that can
be extracted from the Request's path. You can deserialize any variable
segment from the path.

For instance, for resource that registered for the `/users/{userid}/{friend}` path
two segments could be deserialized, `userid` and `friend`. These segments 
could be extracted into a `tuple`, i.e. `Path<(u32, String)>` or any structure
that implements the `Deserialize` trait from the *serde* crate.

```rust
use actix_web::{App, Path, Result, http};

/// extract path info from "/users/{userid}/{friend}" url
/// {userid} -  - deserializes to a u32
/// {friend} - deserializes to a String
fn index(info: Path<(u32, String)>) -> Result<String> {
    Ok(format!("Welcome {}! {}", info.1, info.0))
}

fn main() {
    let app = App::new().resource(
        "/users/{userid}/{friend}",                    // <- define path parameters
        |r| r.method(http::Method::GET).with(index));  // <- use `with` extractor
}
```

Remember! A handler function that uses extractors has to be registered using the 
[*Route::with()*](../../actix-web/actix_web/dev/struct.Route.html#method.with) method.

It is also possible to extract path information to a specific type that
implements the `Deserialize` trait from *serde*. Here is an equivalent example that uses *serde*
instead of a *tuple* type.

```rust
#[macro_use] extern crate serde_derive;
use actix_web::{App, Path, Result, http};

#[derive(Deserialize)]
struct Info {
    userid: u32,
    friend: String,
}

/// extract path info using serde
fn index(info: Path<Info>) -> Result<String> {
     Ok(format!("Welcome {}!", info.friend))
}

fn main() {
    let app = App::new().resource(
       "/users/{userid}/{friend}",                    // <- define path parameters
       |r| r.method(http::Method::GET).with(index));  // <- use `with` extractor
}
```

# Query

Same can be done with the request's query.
The [*Query*](../../actix-web/actix_web/struct.Query.html)
type provides extraction functionality. Underneath it uses *serde_urlencoded* crate.

```rust
#[macro_use] extern crate serde_derive;
use actix_web::{App, Query, http};

#[derive(Deserialize)]
struct Info {
    username: String,
}

// this handler get called only if the request's query contains `username` field
fn index(info: Query<Info>) -> String {
    format!("Welcome {}!", info.username)
}

fn main() {
    let app = App::new().resource(
       "/index.html",
       |r| r.method(http::Method::GET).with(index)); // <- use `with` extractor
}
```

# Json

[*Json*](../../actix-web/actix_web/struct.Json.html) allows to deserialize
a request body into a struct. To extract typed information from a request's body,
the type `T` must implement the `Deserialize` trait from *serde*.

```rust
#[macro_use] extern crate serde_derive;
use actix_web::{App, Json, Result, http};

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// deserialize `Info` from request's body
fn index(info: Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

fn main() {
    let app = App::new().resource(
       "/index.html",
       |r| r.method(http::Method::POST).with(index));  // <- use `with` extractor
}
```

Some extractors provide a way to configure the extraction process. Json extractor
[*JsonConfig*](../../actix-web/actix_web/dev/struct.JsonConfig.html) type for configuration.
When you register a handler using `Route::with()`, it returns a configuration instance. In case of
a *Json* extractor it returns a *JsonConfig*. You can configure the maximum size of the json
payload as well as a custom error handler function.

The following example limits the size of the payload to 4kb and uses a custom error handler.

```rust
#[macro_use] extern crate serde_derive;
use actix_web::{App, Json, HttpResponse, Result, http, error};

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// deserialize `Info` from request's body, max payload size is 4kb
fn index(info: Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

fn main() {
    let app = App::new().resource(
       "/index.html", |r| {
           r.method(http::Method::POST)
              .with(index)
              .limit(4096)   // <- change json extractor configuration
              .error_handler(|err, req| {  // <- create custom error response
                  error::InternalError::from_response(
                     err, HttpResponse::Conflict().finish()).into()
              });
       });
}
```

# Form

At the moment only url-encoded forms are supported. The url-encoded body
could be extracted to a specific type. This type must implement
the `Deserialize` trait from the *serde* crate.

[*FormConfig*](../../actix-web/actix_web/dev/struct.FormConfig.html) allows
configuring the extraction process.

```rust
#[macro_use] extern crate serde_derive;
use actix_web::{App, Form, Result};

#[derive(Deserialize)]
struct FormData {
    username: String,
}

/// extract form data using serde
/// this handler gets called only if the content type is *x-www-form-urlencoded*
/// and the content of the request could be deserialized to a `FormData` struct
fn index(form: Form<FormData>) -> Result<String> {
     Ok(format!("Welcome {}!", form.username))
}
# fn main() {}
```

# Multiple extractors

Actix provides extractor implementations for tuples (up to 10 elements)
whose elements implement `FromRequest`.

For example we can use a path extractor and a query extractor at the same time.

```rust
#[macro_use] extern crate serde_derive;
use actix_web::{App, Query, Path, http};

#[derive(Deserialize)]
struct Info {
    username: String,
}

fn index((path, query): (Path<(u32, String)>, Query<Info>)) -> String {
    format!("Welcome {}!", query.username)
}

fn main() {
    let app = App::new().resource(
       "/users/{userid}/{friend}",                    // <- define path parameters
       |r| r.method(http::Method::GET).with(index)); // <- use `with` extractor
}
```

# Other

Actix also provides several other extractors:

* [*State*](../../actix-web/actix_web/struct.State.html) - If you need
  access to an application state. This is similar to a `HttpRequest::state()`.
* *HttpRequest* - *HttpRequest* itself is an extractor which returns self,
  in case you need access to the request.
* *String* - You can convert a request's payload to a *String*.
  [*Example*](../../actix-web/actix_web/trait.FromRequest.html#example-1)
  is available in doc strings.
* *bytes::Bytes* - You can convert a request's payload into *Bytes*.
  [*Example*](../../actix-web/actix_web/trait.FromRequest.html#example)
  is available in doc strings.
