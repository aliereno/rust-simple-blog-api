use actix_web::HttpServer;
use app::create_app;
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, SqliteConnection};
use dotenv::dotenv;
use env_logger::Env;

mod api;
mod app;
mod db;
mod models;
mod service;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let db_pool: db::DBPool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    db::run_migration_from_pool(db_pool.clone());

    HttpServer::new(move || create_app(&db_pool))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
