use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub mod schema;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn run_migration(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

#[cfg(test)]
pub mod test_db;
