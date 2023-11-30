use actix_web::web;
use diesel::SqliteConnection;

use crate::db::DBPool;

use super::error::ApiError;

pub async fn get_conn(
    db: web::Data<DBPool>,
) -> Result<diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>, ApiError> {
    db.get().map_err(|_| {
        ApiError::new(
            "Failed to get DB connection",
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        )
    })
}
