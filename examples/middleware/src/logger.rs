// <logger>
use actix_web::middleware::Logger;
use actix_web::App;
use env_logger;

pub fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    App::new()
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"));
}
// </logger>
