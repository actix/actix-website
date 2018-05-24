#![allow(dead_code)]
use actix_web::{http::Method, App, HttpRequest};

fn get_projects(_: HttpRequest) -> String {
    unimplemented!()
}
fn create_project(_: HttpRequest) -> String {
    unimplemented!()
}
fn update_project(_: HttpRequest) -> String {
    unimplemented!()
}
fn delete_project(_: HttpRequest) -> String {
    unimplemented!()
}
fn get_tasks(_: HttpRequest) -> String {
    unimplemented!()
}
fn create_task(_: HttpRequest) -> String {
    unimplemented!()
}
fn update_task(_: HttpRequest) -> String {
    unimplemented!()
}
fn delete_task(_: HttpRequest) -> String {
    unimplemented!()
}

fn main() {
// <scope>
App::new().scope("/project", |proj_scope| {
    proj_scope
        .resource("", |r| {
            r.method(Method::GET).f(get_projects);
            r.method(Method::POST).f(create_project)
        })
        .resource("/{project_id}", |r| {
            r.method(Method::PUT).with(update_project);
            r.method(Method::DELETE).f(delete_project)
        })
        .nested("/{project_id}/task", |task_scope| {
            task_scope
                .resource("", |r| {
                    r.method(Method::GET).f(get_tasks);
                    r.method(Method::POST).f(create_task)
                })
                .resource("/{task_id}", |r| {
                    r.method(Method::PUT).with(update_task);
                    r.method(Method::DELETE).with(delete_task)
                })
        })
});
// </scope>
}
