// <config-two>
// use actix_files::{FileConfig, Files};
// use actix_web::App;

// #[derive(Default)]
// struct MyConfig;

// impl FileConfig for MyConfig {
//     fn is_use_etag() -> bool {
//         false
//     }

//     fn is_use_last_modifier() -> bool {
//         false
//     }
// }

// fn main() {
//     App::new().service(
//         "/static",
//         Files::with_config(".", MyConfig)
//             .unwrap()
//             .show_files_listing(),
//     );
// }
// </config-two>
