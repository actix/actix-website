---
title: 数据库
menu: docs_patterns
weight: 1010
---

# Diesel

目前，Diesel 1.0不支持异步操作，但可以将actix同步actor系统用作数据库接口API。从技术上讲，同步actor是worker风格的actor。
多个同步actors可以并行运行并处理来自同一队列的消息。同步actors以mpsc模式工作。

我们来创建一个简单的数据库api，它可以将一个新的 user row插入到SQLite表中。我们必须定义一个同步actor和该actor将使用的连接。其他数据库可以使用相同的方法。

```rust
use actix::prelude::*;

struct DbExecutor(SqliteConnection);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
```

这是我们actor的定义。现在，我们必须定义创建用户消息和响应。

```rust
struct CreateUser {
    name: String,
}

impl Message for CreateUser {
    type Result = Result<User, Error>;
}
```

我们可以向演员发送`CreateUser`消息`DbExecutor` actor，因此我们将收到一个 `User`实例。接下来，我们必须为此消息定义处理程序实现。

```rust
impl Handler<CreateUser> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result
    {
        use self::schema::users::dsl::*;

        // Create insertion model
        let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_user = models::NewUser {
            id: &uuid,
            name: &msg.name,
        };

        // normal diesel operations
        diesel::insert_into(users)
            .values(&new_user)
            .execute(&self.0)
            .expect("Error inserting person");

        let mut items = users
            .filter(id.eq(&uuid))
            .load::<models::User>(&self.0)
            .expect("Error loading person");

        Ok(items.pop().unwrap())
    }
}
```

仅此而已！现在，我们可以使用来在于任何http处理程序或中间件的DbExecutor actor。我们需要的只是启动DbExecutor actors并将地址存储在http处理程序可以访问的状态中。

```rust
/// This is state where we will store *DbExecutor* address.
struct State {
    db: Addr<Syn, DbExecutor>,
}

fn main() {
    let sys = actix::System::new("diesel-example");

    // Start 3 parallel db executors
    let addr = SyncArbiter::start(3, || {
        DbExecutor(SqliteConnection::establish("test.db").unwrap())
    });

    // Start http server
    HttpServer::new(move || {
        App::with_state(State{db: addr.clone()})
            .resource("/{name}", |r| r.method(Method::GET).a(index))})
        .bind("127.0.0.1:8080").unwrap()
        .start().unwrap();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
```

我们将在请求处理程序中使用该地址。处理程序返回future对象; 因此，我们异步接收响应消息。`Route::a()`必须用于异步处理注册。

```rust
/// Async handler
fn index(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let name = &req.match_info()["name"];

    // Send message to `DbExecutor` actor
    req.state().db.send(CreateUser{name: name.to_owned()})
        .from_err()
        .and_then(|res| {
            match res {
                Ok(user) => Ok(HttpResponse::Ok().json(user)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
}
```

[diesel directory](https://github.com/actix/examples/tree/master/diesel/)提供了一个完整的示例。

有关同步actors的更多信息可以在[actix documentation](https://docs.rs/actix/0.5.0/actix/sync/index.html)文档中找到 。
