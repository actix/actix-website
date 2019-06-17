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

{{< include-example example="extractors" file="main.rs" section="main" >}}

## Within Custom Handler Types

Like a handler function, a custom Handler type can *access* an Extractor by
calling the ExtractorType::<...>::extract(&req) function.  An Extractor 
*cannot* be passed as a parameter to a custom Handler type because a custom 
Handler type must follow the ``handle`` function signature specified by the 
Handler trait it implements.

{{< include-example example="extractors" file="custom_handler.rs" section="custom-handler" >}}

# Path

[*Path*](../../actix-web/actix_web/struct.Path.html) provides information that can
be extracted from the Request's path. You can deserialize any variable
segment from the path.

For instance, for resource that registered for the `/users/{userid}/{friend}` path
two segments could be deserialized, `userid` and `friend`. These segments 
could be extracted into a `tuple`, i.e. `Path<(u32, String)>` or any structure
that implements the `Deserialize` trait from the *serde* crate.

{{< include-example example="extractors" file="path_one.rs" section="path-one" >}}

It is also possible to extract path information to a specific type that
implements the `Deserialize` trait from *serde*. Here is an equivalent example that uses *serde*
instead of a *tuple* type.

{{< include-example example="extractors" file="path_two.rs" section="path-two" >}}

# Query

Same can be done with the request's query.
The [*Query*](../../actix-web/actix_web/struct.Query.html)
type provides extraction functionality. Underneath it uses *serde_urlencoded* crate.

{{< include-example example="extractors" file="query.rs" section="query" >}}

# Json

[*Json*](../../actix-web/actix_web/struct.Json.html) allows to deserialize
a request body into a struct. To extract typed information from a request's body,
the type `T` must implement the `Deserialize` trait from *serde*.

{{< include-example example="extractors" file="json_one.rs" section="json-one" >}}

Some extractors provide a way to configure the extraction process. Json extractor
[*JsonConfig*](../../actix-web/actix_web/dev/struct.JsonConfig.html) type for configuration.
When you register a handler using `Route::with()`, it returns a configuration instance. In case of
a *Json* extractor it returns a *JsonConfig*. You can configure the maximum size of the json
payload as well as a custom error handler function.

The following example limits the size of the payload to 4kb and uses a custom error handler.

{{< include-example example="extractors" file="json_two.rs" section="json-two" >}}

# Form

At the moment only url-encoded forms are supported. The url-encoded body
could be extracted to a specific type. This type must implement
the `Deserialize` trait from the *serde* crate.

[*FormConfig*](../../actix-web/actix_web/dev/struct.FormConfig.html) allows
configuring the extraction process.

{{< include-example example="extractors" file="form.rs" section="form" >}}

# Multiple extractors

Actix provides extractor implementations for tuples (up to 10 elements)
whose elements implement `FromRequest`.

For example we can use a path extractor and a query extractor at the same time.

{{< include-example example="extractors" file="multiple.rs" section="multi" >}}

# Other

Actix also provides several other extractors:

* [*Data*](../../actix-web/actix_web/web/struct.Data.html) - If you need
  access to an application state. This is similar to a `HttpRequest::app_data()`.
* *HttpRequest* - *HttpRequest* itself is an extractor which returns self,
  in case you need access to the request.
* *String* - You can convert a request's payload to a *String*.
  [*Example*](../../actix-web/actix_web/trait.FromRequest.html#example-1)
  is available in doc strings.
* *bytes::Bytes* - You can convert a request's payload into *Bytes*.
  [*Example*](../../actix-web/actix_web/trait.FromRequest.html#example)
  is available in doc strings.
