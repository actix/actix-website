---
title: Application
menu: docs_basics
weight: 140
---

# Writing an Application

`actix-web` provides various primitives to build web servers and applications with Rust.
It provides routing, middlewares, pre-processing of requests, post-processing of
responses, etc.

All `actix-web` servers are built around the [`App`][app] instance.  It is used for
registering routes for resources and middlewares.  It also stores application
state shared across all handlers within the same scope.

An application's [`scope`][scope] acts as a namespace for all routes, i.e. all routes for a
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

## State

Application state is shared with all routes and resources within the same scope. State
can be accessed with the [`web::Data<T>`][data] extractor where `T` is the type of the state. State is
also available for middlewares.

Let's write a simple application and store the application name in the state:

{{< include-example example="application" file="state.rs" section="setup" >}}

and pass in the state when initializing the App, and start the application:

{{< include-example example="application" file="state.rs" section="start_app" >}}

Any number of state types could be registered within the application.

## Shared Mutable State

`HttpServer` accepts an application factory rather than an application instance.
An `HttpServer` constructs an application instance for each thread. Therefore, application data must be
constructed multiple times. If you want to share data between different threads, a shareable
object should be used, e.g. `Send` + `Sync`.

Internally, [`web::Data`][data] uses `Arc`. Thus, in order to avoid creating two `Arc`s, we should create our Data before registering it using [`App::app_data()`][appdata].

In the following example, we will write an application with mutable, shared state. First, we define our state and create our handler:

{{< include-example example="application" file="state.rs" section="setup_mutable" >}}

and register the data in an `App`:

{{< include-example example="application" file="state.rs" section="make_app_mutable" >}}

## Using an Application Scope to Compose Applications

The [`web::scope()`][webscope] method allows setting a resource group prefix. This scope represents
a resource prefix that will be prepended to all resource patterns added by the resource
configuration. This can be used to help mount a set of routes at a different location
than the original author intended while still maintaining the same resource names.

For example:

{{< include-example example="application" file="scope.rs" section="scope" >}}

In the above example, the `show_users` route will have an effective route pattern of
`/users/show` instead of `/show` because the application's scope argument will be prepended
to the pattern. The route will then only match if the URL path is `/users/show`,
and when the [`HttpRequest.url_for()`][urlfor] function is called with the route name `show_users`,
it will generate a URL with that same path.

## Application guards and virtual hosting

You can think of a guard as a simple function that accepts a *request* object reference
and returns *true* or *false*. Formally, a guard is any object that implements the
[`Guard`][guardtrait] trait. Actix-web provides several guards. You can check the
[functions section][guardfuncs] of the API docs.

One of the provided guards is [`Header`][guardheader]. It can be used as a
filter based on request header information.

{{< include-example example="application" file="vh.rs" section="vh" >}}

# Configure

For simplicity and reusability both [`App`][appconfig] and [`web::Scope`][webscopeconfig] provide the `configure` method.
This function is useful for moving parts of the configuration to a different module or even
library. For example, some of the resource's configuration could be moved to a different
module.

{{< include-example example="application" file="config.rs" section="config" >}}

The result of the above example would be:

```
/         -> "/"
/app      -> "app"
/api/test -> "test"
```
Each [`ServiceConfig`][serviceconfig] can have its own `data`, `routes`, and `services`.

[usingappprefix]: /docs/url-dispatch/index.html#using-an-application-prefix-to-compose-applications
[stateexample]: https://github.com/actix/examples/blob/master/state/src/main.rs
[guardtrait]: https://docs.rs/actix-web/2/actix_web/guard/trait.Guard.html
[guardfuncs]: https://docs.rs/actix-web/2/actix_web/guard/index.html#functions
[guardheader]: https://docs.rs/actix-web/2/actix_web/guard/fn.Header.html
[data]: https://docs.rs/actix-web/2/actix_web/web/struct.Data.html
[app]: https://docs.rs/actix-web/2/actix_web/struct.App.html
[appconfig]: https://docs.rs/actix-web/2/actix_web/struct.App.html#method.configure
[appdata]: https://docs.rs/actix-web/2/actix_web/struct.App.html#method.app_data
[scope]: https://docs.rs/actix-web/2/actix_web/struct.Scope.html
[webscopeconfig]: https://docs.rs/actix-web/2/actix_web/struct.Scope.html#method.configure
[webscope]: https://docs.rs/actix-web/2/actix_web/web/fn.scope.html
[urlfor]: https://docs.rs/actix-web/2/actix_web/struct.HttpRequest.html#method.url_for
[serviceconfig]: https://docs.rs/actix-web/2/actix_web/web/struct.ServiceConfig.html
