---
title: Application
menu: docs_basics
weight: 140
---

# Writing an Application

`actix-web` provides various primitives to build web servers and applications with Rust.
It provides routing, middlewares, pre-processing of requests, post-processing of responses,
websocket protocol handling, multipart streams, etc.

All actix web servers are built around the `App` instance.  It is used for
registering routes for resources and middlewares.  It also stores application
state shared across all handlers within same application.

Applications act as a namespace for all routes, i.e. all routes for a specific application
have the same url path prefix. The application prefix always contains a leading "/" slash.
If a supplied prefix does not contain leading slash, it is automatically inserted.
The prefix should consist of value path segments.

> For an application with prefix `/app`,
> any request with the paths `/app`, `/app/`, or `/app/test` would match;
> however, the path `/application` would not match.

{{< include-example example="application" section="make_app" >}}

In this example, an application with the `/app` prefix and a `index.html` resource
are created. This resource is available through the `/app/index.html` url.

> For more information, check the
> [URL Dispatch](./sec-6-url-dispatch.html#using-a-application-prefix-to-compose-applications) section.

Multiple applications can be served with one server:

{{< include-example example="application" section="run_server" >}}

All `/app1` requests route to the first application, `/app2` to the second, and all other to the third.
**Applications get matched based on registration order**. If an application with a more generic
prefix is registered before a less generic one, it would effectively block the less generic
application matching. For example, if an `App` with the prefix `"/"` was registered
as the first application, it would match all incoming requests.

## State

Application state is shared with all routes and resources within the same application.
When using an http actor, state can be accessed with the `HttpRequest::state()` as read-only,
but interior mutability with `RefCell` can be used to achieve state mutability.
State is also available for route matching predicates and middlewares.

Let's write a simple application that uses shared state. We are going to store request count
in the state:

{{< include-example example="application" file="state.rs" section="setup" >}}

When the app is initialized it needs to be passed the initial state:

{{< include-example example="application" file="state.rs" section="make_app" >}}

> **Note**: http server accepts an application factory rather than an application
> instance. Http server constructs an application instance for each thread, thus application state
> must be constructed multiple times. If you want to share state between different threads, a
> shared object should be used, e.g. `Arc`. Application state does not need to be `Send` and `Sync`,
> but the application factory must be `Send` + `Sync`.
>
> To start the previous app, create it into a closure:

{{< include-example example="application" file="state.rs" section="start_app" >}}

## Combining applications with different state

Combining multiple applications with different state is possible as well.

[server::new](https://docs.rs/actix-web/*/actix_web/server/fn.new.html) requires the handler to have a single type. 

This limitation can easily be overcome with the [App::boxed](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.boxed) method, which converts an App into a boxed trait object.

{{< include-example example="application" file="state.rs" section="combine" >}}

## Using an Application Prefix to Compose Applications

The `App::prefix()` method allows to set a specific application prefix.
This prefix represents a resource prefix that will be prepended to all resource patterns added
by the resource configuration. This can be used to help mount a set of routes at a different
location than the included callable's author intended while still maintaining the same
resource names.

For example:

{{< include-example example="url-dispatch" file="prefix.rs" section="prefix" >}}

In the above example, the *show_users* route will have an effective route pattern of
*/users/show* instead of */show* because the application's prefix argument will be prepended
to the pattern. The route will then only match if the URL path is */users/show*,
and when the `HttpRequest.url_for()` function is called with the route name show_users,
it will generate a URL with that same path.

## Application predicates and virtual hosting

You can think of a predicate as a simple function that accepts a *request* object reference
and returns *true* or *false*. Formally, a predicate is any object that implements the
[`Predicate`](../actix_web/pred/trait.Predicate.html) trait. Actix provides
several predicates, you can check
[functions section](../../actix-web/actix_web/pred/index.html#functions) of api docs.

Any of this predicates could be used 
with [`App::filter()`](../actix_web/struct.App.html#method.filter) method. One of the
provided predicates is [`Host`](../actix_web/pred/fn.Host.html), it can be used
as application's filter based on request's host information.

{{< include-example example="application" file="vh.rs" section="vh" >}}
