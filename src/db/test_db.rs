use diesel::{r2d2::ConnectionManager, r2d2::Pool, sql_query, Connection, PgConnection, RunQueryDsl};
use std::sync::atomic::AtomicU32;
use url::Url;

use crate::db::run_migration;

use super::DBPool;

static TEST_DB_COUNTER: AtomicU32 = AtomicU32::new(0);

// https://github.com/diesel-rs/diesel/issues/1549#issuecomment-892978784
pub struct TestDb {
    default_db_url: String,
    url: String,
    name: String,
    delete_on_drop: bool,
}
impl TestDb {
    pub fn new() -> Self {
        let name = format!(
            "test_db_{}_{}",
            std::process::id(),
            TEST_DB_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        );
        let default_db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
        let mut conn = PgConnection::establish(&default_db_url).unwrap();

        sql_query(format!("CREATE DATABASE {};", name))
            .execute(&mut conn)
            .unwrap();

        let mut url = Url::parse(&default_db_url).unwrap();
        url.set_path(&name);

        let mut conn = PgConnection::establish(&url.to_string()).unwrap();
        run_migration(&mut conn);

        Self {
            default_db_url,
            url: url.to_string(),
            name,
            delete_on_drop: true,
        }
    }

    pub fn conn(&self) -> DBPool {
        let manager = ConnectionManager::<PgConnection>::new(&self.url);
        let db_pool: DBPool = Pool::builder().build(manager).expect("Failed to create pool.");

        return db_pool;
    }
}
impl Drop for TestDb {
    fn drop(&mut self) {
        if !self.delete_on_drop {
            println!("TestDb leaking database {}", self.name);
            return;
        }
        let mut conn = PgConnection::establish(&self.default_db_url).unwrap();

        sql_query(format!(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}'",
            self.name
        ))
        .execute(&mut conn)
        .unwrap();

        sql_query(format!("DROP DATABASE {}", self.name))
            .execute(&mut conn)
            .unwrap();
    }
}
