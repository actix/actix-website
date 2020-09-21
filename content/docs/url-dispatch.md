---
title: URL Dispatch
menu: docs_advanced
weight: 190
---

# URL Dispatch

URL dispatch provides a simple way for mapping URLs to handler code using a simple pattern
matching language. If one of the patterns matches the path information associated with a request,
a particular handler object is invoked.

> A request handler is a function that accepts zero or more parameters that can be extracted
> from a request (ie, [*impl FromRequest*][implfromrequest]) and returns a type that can
> be converted into an HttpResponse (ie, [*impl Responder*][implresponder]).  More information
> is available in the [handler section][handlersection].

# Resource configuration

Resource configuration is the act of adding a new resources to an application.  A resource
has a name, which acts as an identifier to be used for URL generation.  The name also
allows developers to add routes to existing resources.  A resource also has a pattern,
meant to match against the *PATH* portion of a *URL* (the portion following the scheme and
port, e.g. */foo/bar* in the *URL* *http://localhost:8080/foo/bar?q=value*).  It does not
match against the *QUERY* portion (the portion that follows *?*, e.g. *q=value*
in *http://localhost:8080/foo/bar?q=value*).

The [*App::route()*][approute] method provides simple way of registering routes. This
method adds a single route to application routing table. This method accepts a *path pattern*,
*HTTP method* and a handler function. `route()` method could be called multiple times
for the same path, in that case, multiple routes register for the same resource path.

{{< include-example example="url-dispatch" section="main" >}}

While *App::route()* provides simple way of registering routes, to access complete resource
configuration, a different method has to be used.  The [*App::service()*][appservice] method
adds a single [resource][webresource] to application routing table. This method accepts a
*path pattern*, guards, and one or more routes.

{{< include-example example="url-dispatch" file="resource.rs" section="resource" >}}

If a resource does not contain any route or does not have any matching routes, it
returns *NOT FOUND* HTTP response.

## Configuring a Route

Resource contains a set of routes. Each route in turn has a set of `guards` and a handler.
New routes can be created with `Resource::route()` method which returns a reference
to new *Route* instance. By default the *route* does not contain any guards, so matches
all requests and the default handler is `HttpNotFound`.

The application routes incoming requests based on route criteria which are defined during
resource registration and route registration. Resource matches all routes it contains in
the order the routes were registered via `Resource::route()`.

> A *Route* can contain any number of *guards* but only one handler.

{{< include-example example="url-dispatch" file="cfg.rs" section="cfg" >}}

In this example, `HttpResponse::Ok()` is returned for *GET* requests if the request
contains `Content-Type` header, the value of this header is *text/plain*, and path
equals to `/path`.

If a resource can not match any route, a "NOT FOUND" response is returned.

[*ResourceHandler::route()*][resourcehandler] returns a [*Route*][route] object. Route
can be configured with a builder-like pattern. Following configuration methods are available:

* [*Route::guard()*][routeguard] registers a new guard. Any number of guards can be
  registered for each route.
* [*Route::method()*][routemethod] registers a method guard. Any number of guards can be
  registered for each route.
* [*Route::to()*][routeto] registers an async handler function for this route.
  Only one handler can be registered.  Usually handler registration is the last config operation.

# Route matching

The main purpose of route configuration is to match (or not match) the request's `path`
against a URL path pattern. `path` represents the path portion of the URL that was requested.

The way that *actix-web* does this is very simple. When a request enters the system,
for each resource configuration declaration present in the system, actix checks
the request's path against the pattern declared. This checking happens in the order that
the routes were declared via `App::service()` method. If resource can not be found,
the *default resource* is used as the matched resource.

When a route configuration is declared, it may contain route guard arguments. All route
guards associated with a route declaration must be `true` for the route configuration to
be used for a given request during a check. If any guard in the set of route guard
arguments provided to a route configuration returns `false` during a check, that route is
skipped and route matching continues through the ordered set of routes.

If any route matches, the route matching process stops and the handler associated with
the route is invoked. If no route matches after all route patterns are exhausted, a
*NOT FOUND* response get returned.

# Resource pattern syntax

The syntax of the pattern matching language used by actix in the pattern
argument is straightforward.

The pattern used in route configuration may start with a slash character. If the pattern
does not start with a slash character, an implicit slash will be prepended
to it at matching time. For example, the following patterns are equivalent:

```
{foo}/bar/baz
```

and:

```
/{foo}/bar/baz
```

A *variable part* (replacement marker) is specified in the form *{identifier}*,
where this means "accept any characters up to the next slash character and use this
as the name in the `HttpRequest.match_info()` object".

A replacement marker in a pattern matches the regular expression `[^{}/]+`.

A match_info is the `Params` object representing the dynamic parts extracted from a
*URL* based on the routing pattern. It is available as *request.match_info*. For example, the
following pattern defines one literal segment (foo) and two replacement markers (baz, and bar):

```
foo/{baz}/{bar}
```

The above pattern will match these URLs, generating the following match information:

```
foo/1/2        -> Params {'baz':'1', 'bar':'2'}
foo/abc/def    -> Params {'baz':'abc', 'bar':'def'}
```

It will not match the following patterns however:

```
foo/1/2/        -> No match (trailing slash)
bar/abc/def     -> First segment literal mismatch
```

The match for a segment replacement marker in a segment will be done only up to
the first non-alphanumeric character in the segment in the pattern. So, for instance,
if this route pattern was used:

```
foo/{name}.html
```

The literal path */foo/biz.html* will match the above route pattern, and the match result
will be `Params{'name': 'biz'}`. However, the literal path */foo/biz* will not match,
because it does not contain a literal *.html* at the end of the segment represented
by *{name}.html* (it only contains biz, not biz.html).

To capture both segments, two replacement markers can be used:

```
foo/{name}.{ext}
```

The literal path */foo/biz.html* will match the above route pattern, and the match
result will be *Params{'name': 'biz', 'ext': 'html'}*. This occurs because there is a
literal part of *.* (period) between the two replacement markers *{name}* and *{ext}*.

Replacement markers can optionally specify a regular expression which will be used to decide
whether a path segment should match the marker. To specify that a replacement marker should
match only a specific set of characters as defined by a regular expression, you must use a
slightly extended form of replacement marker syntax. Within braces, the replacement marker
name must be followed by a colon, then directly thereafter, the regular expression. The default
regular expression associated with a replacement marker *[^/]+* matches one or more characters
which are not a slash. For example, under the hood, the replacement marker *{foo}* can more
verbosely be spelled as *{foo:[^/]+}*. You can change this to be an arbitrary regular expression
to match an arbitrary sequence of characters, such as *{foo:\d+}* to match only digits.

Segments must contain at least one character in order to match a segment replacement marker.
For example, for the URL */abc/*:

* */abc/{foo}* will not match.
* */{foo}/* will match.

> **Note**: path will be URL-unquoted and decoded into valid unicode string before
> matching pattern and values representing matched path segments will be URL-unquoted too.

So for instance, the following pattern:

```
foo/{bar}
```

When matching the following URL:

```
http://example.com/foo/La%20Pe%C3%B1a
```

The match dictionary will look like so (the value is URL-decoded):

```
Params{'bar': 'La Pe\xf1a'}
```

Literal strings in the path segment should represent the decoded value of the
path provided to actix. You don't want to use a URL-encoded value in the pattern.
For example, rather than this:

```
/Foo%20Bar/{baz}
```

You'll want to use something like this:

```
/Foo Bar/{baz}
```

It is possible to get "tail match". For this purpose custom regex has to be used.

```
foo/{bar}/{tail:.*}
```

The above pattern will match these URLs, generating the following match information:

```
foo/1/2/           -> Params{'bar':'1', 'tail': '2/'}
foo/abc/def/a/b/c  -> Params{'bar':u'abc', 'tail': 'def/a/b/c'}
```

# Scoping Routes

Scoping helps you organize routes sharing common root paths.  You can nest
scopes within scopes.

Suppose that you want to organize paths to endpoints used to view "Users". Such paths may include:

- /users
- /users/show
- /users/show/{id}


A scoped layout of these paths would appear as follows

{{< include-example example="url-dispatch" file="scope.rs" section="scope" >}}

A *scoped* path can contain variable path segments as resources. Consistent with 
un-scoped paths.

You can get variable path segments from `HttpRequest::match_info()`.
[`Path` extractor][pathextractor] also is able to extract scope level variable segments.

# Match information

All values representing matched path segments are available in [`HttpRequest::match_info`][matchinfo].
Specific values can be retrieved with [`Path::get()`][pathget].

{{< include-example example="url-dispatch" file="minfo.rs" section="minfo" >}}

For this example for path '/a/1/2/', values v1 and v2 will resolve to "1" and "2".

It is possible to create a `PathBuf` from a tail path parameter. The returned `PathBuf` is
percent-decoded. If a segment is equal to "..", the previous segment (if
any) is skipped.

For security purposes, if a segment meets any of the following conditions,
an `Err` is returned indicating the condition met:

* Decoded segment starts with any of: `.` (except `..`), `*`
* Decoded segment ends with any of: `:`, `>`, `<`
* Decoded segment contains any of: `/`
* On Windows, decoded segment contains any of: '\'
* Percent-encoding results in invalid UTF8.

As a result of these conditions, a `PathBuf` parsed from request path parameter is
safe to interpolate within, or use as a suffix of, a path without additional checks.

{{< include-example example="url-dispatch" file="pbuf.rs" section="pbuf" >}}

## Path information extractor

Actix provides functionality for type safe path information extraction.  [*Path*][pathstruct]
extracts information, destination type could be defined in several different forms. Simplest
approach is to use `tuple` type. Each element in tuple must correspond to one element from
path pattern. i.e. you can match path pattern `/{id}/{username}/` against
`Path<(u32, String)>` type, but `Path<(String, String, String)>` type will always fail.

{{< include-example example="url-dispatch" file="path.rs" section="path" >}}

It also possible to extract path pattern information to a struct. In this case,
this struct must implement *serde's *`Deserialize` trait.

{{< include-example example="url-dispatch" file="path2.rs" section="path" >}}

[*Query*][query] provides similar functionality for request query parameters.

# Generating resource URLs

Use the [*HttpRequest.url_for()*][urlfor] method to generate URLs based on resource
patterns. For example, if you've configured a resource with the name "foo" and the
pattern "{a}/{b}/{c}", you might do this:

{{< include-example example="url-dispatch" file="urls.rs" section="url" >}}

This would return something like the string *http://example.com/test/1/2/3* (at least if
the current protocol and hostname implied http://example.com).  `url_for()` method
returns [*Url object*][urlobj] so you can modify this url (add query parameters, anchor, etc).
`url_for()` could be called only for *named* resources otherwise error get returned.

# External resources

Resources that are valid URLs, can be registered as external resources. They are useful
for URL generation purposes only and are never considered for matching at request time.

{{< include-example example="url-dispatch" file="url_ext.rs" section="ext" >}}

# Path normalization and redirecting to slash-appended routes

By normalizing it means:

* To add a trailing slash to the path.
* To replace multiple slashes with one.

The handler returns as soon as it finds a path that resolves correctly. The order of
normalization conditions, if all are enabled, is 1) merge, 2) both merge and append and
3) append. If the path resolves with at least one of those conditions, it will redirect
to the new path.

{{< include-example example="url-dispatch" file="norm.rs" section="norm" >}}

In this example `//resource///` will be redirected to `/resource/`.

In this example, the path normalization handler is registered for all methods,
but you should not rely on this mechanism to redirect *POST* requests. The redirect of the
slash-appending *Not Found* will turn a *POST* request into a GET, losing any
*POST* data in the original request.

It is possible to register path normalization only for *GET* requests only:

{{< include-example example="url-dispatch" file="norm2.rs" section="norm" >}}

## Using an Application Prefix to Compose Applications

The `web::scope()` method allows to set a specific application scope.  This scope represents
a resource prefix that will be prepended to all resource patterns added by the resource
configuration. This can be used to help mount a set of routes at a different location
than the included callable's author intended while still maintaining the same resource names.

For example:

{{< include-example example="url-dispatch" file="scope.rs" section="scope" >}}

In the above example, the *show_users* route will have an effective route pattern of
*/users/show* instead of */show* because the application's scope will be prepended
to the pattern. The route will then only match if the URL path is */users/show*,
and when the `HttpRequest.url_for()` function is called with the route name show_users,
it will generate a URL with that same path.

# Custom route guard

You can think of a guard as a simple function that accepts a *request* object reference
and returns *true* or *false*. Formally, a guard is any object that implements the
[`Guard`][guardtrait] trait. Actix provides several predicates, you can check
[functions section][guardfuncs] of API docs.

Here is a simple guard that check that a request contains a specific *header*:

{{< include-example example="url-dispatch" file="guard.rs" section="guard" >}}

In this example, *index* handler will be called only if request contains *CONTENT-TYPE* header.

Guards can not access or modify the request object, but it is possible to store extra
information in [request extensions][requestextensions].

## Modifying guard values

You can invert the meaning of any predicate value by wrapping it in a `Not` predicate.
For example, if you want to return "METHOD NOT ALLOWED" response for all methods
except "GET":

{{< include-example example="url-dispatch" file="guard2.rs" section="guard2" >}}

The `Any` guard accepts a list of guards and matches if any of the supplied
guards match. i.e:

```rust
guard::Any(guard::Get()).or(guard::Post())
```

The `All` guard accepts a list of guard and matches if all of the supplied
guards match. i.e:

```rust
guard::All(guard::Get()).and(guard::Header("content-type", "plain/text"))
```

# Changing the default Not Found response

If the path pattern can not be found in the routing table or a resource can not find matching
route, the default resource is used. The default response is *NOT FOUND*.
It is possible to override the *NOT FOUND* response with `App::default_service()`.
This method accepts a *configuration function* same as normal resource configuration
with `App::service()` method.

{{< include-example example="url-dispatch" file="dhandler.rs" section="default" >}}

[handlersection]: ../handlers/
[approute]: https://docs.rs/actix-web/3/actix_web/struct.App.html#method.route
[appservice]: https://docs.rs/actix-web/3/actix_web/struct.App.html?search=#method.service
[webresource]: https://docs.rs/actix-web/3/actix_web/struct.Resource.html
[resourcehandler]: https://docs.rs/actix-web/3/actix_web/struct.Resource.html#method.route
[route]: https://docs.rs/actix-web/3/actix_web/struct.Route.html
[routeguard]: https://docs.rs/actix-web/3/actix_web/struct.Route.html#method.guard
[routemethod]: https://docs.rs/actix-web/3/actix_web/struct.Route.html#method.method
[routeto]: https://docs.rs/actix-web/3/actix_web/struct.Route.html#method.to
[matchinfo]: https://docs.rs/actix-web/3/actix_web/struct.HttpRequest.html#method.match_info
[pathget]: https://docs.rs/actix-web/3/actix_web/dev/struct.Path.html#method.get
[pathstruct]: https://docs.rs/actix-web/3/actix_web/dev/struct.Path.html
[query]: https://docs.rs/actix-web/3/actix_web/web/struct.Query.html
[urlfor]: https://docs.rs/actix-web/3/actix_web/struct.HttpRequest.html#method.url_for
[urlobj]: https://docs.rs/url/1.7.2/url/struct.Url.html
[guardtrait]: https://docs.rs/actix-web/3/actix_web/guard/trait.Guard.html
[guardfuncs]: https://docs.rs/actix-web/3/actix_web/guard/index.html#functions
[requestextensions]: https://docs.rs/actix-web/3/actix_web/struct.HttpRequest.html#method.extensions
[implfromrequest]: https://docs.rs/actix-web/3/actix_web/trait.FromRequest.html
[implresponder]: https://docs.rs/actix-web/3/actix_web/trait.Responder.html
[pathextractor]: ../extractors
