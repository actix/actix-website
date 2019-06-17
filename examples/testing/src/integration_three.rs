// <integration-three>
// #[test]
// fn test() {
//     let srv = TestServer::build_with_state(|| {
//         // we can start diesel actors
//         let addr = SyncArbiter::start(3, || {
//             DbExecutor(SqliteConnection::establish("test.db").unwrap())
//         });
//         // then we can construct custom state, or it could be `()`
//         MyState{addr: addr}
//    })

//    // register server handlers and start test server
//    .start(|app| {
//         app.resource(
//             "/{username}/index.html", |r| r.with(
//                 |p: Path<PParam>| format!("Welcome {}!", p.username)));
//     });
//     // now we can run our test code
// );
// </integration-three>
