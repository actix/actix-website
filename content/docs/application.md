---
title: Application
menu: docs_basics
weight: 140
---

# Writing an Application

`actix-web` provides various primitives to build web servers and applications with Rust.
It provides routing, middlewares, pre-processing of requests, post-processing of
responses, etc.

All `actix-web` servers are built around the `App` instance.  It is used for
registering routes for resources and middlewares.  It also stores application
state shared across all handlers within same scope.

An application's `scope` acts as a namespace for all routes, i.e. all routes for a
specific application scope have the same url path prefix. The application prefix always
contains a leading "/" slash.  If a supplied prefix does not contain leading slash,
it is automatically inserted.  The prefix should consist of value path segments.

> For an application with scope `/app`,
> any request with the paths `/app`, `/app/`, or `/app/test` would match;
> however, the path `/application` would not match.

{{< include-example example="application" file="app.rs" section="setup" >}}

In this example, an application with the `/app` prefix and a `index.html` resource
are created. This resource is available through the `/app/index.html` url.

> For more information, check the [URL Dispatch][usingappprefix] section.

Multiple application scopes can be served with one server:

{{< include-example example="application" file="main.rs" section="multi" >}}

All `/app1` requests route to the first application, `/app2` to the second, and all other to the third.
**Applications get matched based on registration order**. If an application with a more generic
prefix is registered before a less generic one, it would effectively block the less generic
application matching. For example, if an `App` with the prefix `"/"` was registered
as the first application, it would match all incoming requests.

## State

Application state is shared with all routes and resources within the same scope. State
can be accessed with the `web::Data<State>` extractor. State is also available for route matching guards and middlewares.

Let's write a simple application and store the application name in the state:

{{< include-example example="application" file="state.rs" section="setup" >}}

and pass in the state when initializing the App, and start the application:

{{< include-example example="application" file="state.rs" section="start_app" >}}

## Shared Mutable State

`HttpServer` accepts an application factory rather than an application instance. 
Http server constructs an application instance for each thread, thus application data must be 
constructed multiple times. If you want to share data between different threads, a shareable 
object should be used, e.g. Send + Sync. 

Internally, `web::Data` uses Arc. Thus, in order to avoid double Arc, we should create our Data before registering it using `register_data()`.

In the following example, we will write an application with mutable, shared state. First, we define our state and create our handler:

{{< include-example example="application" file="state.rs" section="setup_mutable" >}}

and register the data in an App:

{{< include-example example="application" file="state.rs" section="make_app_mutable" >}}

## Combining applications with different state

Combining multiple applications with different state is possible as well.

{{< include-example example="application" file="combine.rs" section="combine" >}}

## Using an Application Scope to Compose Applications

The `web::scope()` method allows to set a specific application prefix.  This scope represents
a resource prefix that will be prepended to all resource patterns added by the resource
configuration. This can be used to help mount a set of routes at a different location
than the included callable's author intended while still maintaining the same resource names.

For example:

{{< include-example example="application" file="scope.rs" section="scope" >}}

In the above example, the *show_users* route will have an effective route pattern of
*/users/show* instead of */show* because the application's scope argument will be prepended
to the pattern. The route will then only match if the URL path is */users/show*,
and when the `HttpRequest.url_for()` function is called with the route name show_users,
it will generate a URL with that same path.

## Application guards and virtual hosting

You can think of a guard as a simple function that accepts a *request* object reference
and returns *true* or *false*. Formally, a guard is any object that implements the
[`Guard`][guardtrait] trait. Actix-web provides several guards, you can check
[functions section][guardfuncs] of api docs.

One of the provided guards is [`Header`][guardheader], it can be used as application's
filter based on request's header information.

{{< include-example example="application" file="vh.rs" section="vh" >}}

# Configure

For simplicity and reusability both `App` and `web::scope` provide the `configure` method.
This function is useful for moving parts of configuration to a different module or even
library. For example, some of the resource's configuration could be moved to different
module.

{{< include-example example="application" file="config.rs" section="config" >}}

The result of the above example would be:

```
/         -> "/"
/app      -> "app"
/api/test -> "test"
```

Each `ServiceConfig` can have it's own `data`, `routes`, and `services`

[usingappprefix]: /docs/url-dispatch/index.html#using-an-application-prefix-to-compose-applications
[stateexample]: https://github.com/actix/examples/blob/master/state/src/main.rs
[guardtrait]: https://docs.rs/actix-web/2/actix_web/guard/trait.Guard.html
[guardfuncs]: https://docs.rs/actix-web/2/actix_web/guard/index.html#functions
[guardheader]: https://docs.rs/actix-web/2/actix_web/guard/fn.Header.html
