use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use env_logger::Env;

mod api;
mod db;
mod models;
mod service;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let db_pool: db::DBPool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .configure(api::config_app)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
