use diesel::{
    r2d2::{self, ConnectionManager},
    SqliteConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub mod schema;

pub type DBPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn run_migration(conn: &mut SqliteConnection) {
    let err = conn.run_pending_migrations(MIGRATIONS);
    if err.is_err() {
        println!("{:?}", err.unwrap_err());
    }
}

pub fn run_migration_from_pool(db_pool: DBPool) {
    if let Ok(mut conn) = db_pool.get() {
        run_migration(&mut conn);
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::atomic::AtomicU32;

    use diesel::{
        r2d2::{ConnectionManager, Pool},
        SqliteConnection,
    };

    use super::{run_migration_from_pool, DBPool};
    static TEST_DB_COUNTER: AtomicU32 = AtomicU32::new(0);

    pub fn get_test_db() -> DBPool {
        let url = format!(
            "file:test_db_{}_{}?mode=memory",
            std::process::id(),
            TEST_DB_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        );

        let manager = ConnectionManager::<SqliteConnection>::new(url);
        let db_pool: DBPool = Pool::builder().build(manager).expect("Failed to create pool.");
        run_migration_from_pool(db_pool.clone());

        db_pool
    }
}
