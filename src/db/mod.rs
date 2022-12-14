use diesel::{PgConnection, r2d2::{self, ConnectionManager}};

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub mod schema;