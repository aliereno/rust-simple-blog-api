use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .configure(api::config_app)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}