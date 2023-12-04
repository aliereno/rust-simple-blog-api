use actix_web::{post, web, HttpResponse, Result};

use crate::api::serializers::{LoginIn, LoginOut, MessageOut, RegisterIn};
use crate::db::DBPool;
use crate::service::auth_service;
use crate::utils::connection::get_conn;
use crate::utils::error::ApiError;

pub fn config_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(register);
}

#[post("/register")]
pub async fn register(data: web::Json<RegisterIn>, db_pool: web::Data<DBPool>) -> Result<HttpResponse, ApiError> {
    let mut conn: diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::prelude::SqliteConnection>> =
        get_conn(db_pool).await?;

    auth_service::user_register(&mut conn, &data.username, &data.email, &data.password).await?;

    let response = MessageOut {
        message: "Successfuly registered.".to_string(),
    };
    Ok(HttpResponse::Ok().json(response))
}

#[post("/login")]
pub async fn login(data: web::Json<LoginIn>, db_pool: web::Data<DBPool>) -> Result<HttpResponse, ApiError> {
    let mut conn: diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::prelude::SqliteConnection>> =
        get_conn(db_pool).await?;

    let _access_token = auth_service::user_login(&mut conn, &data.username, &data.password).await?;

    let response = LoginOut {
        access_token: _access_token,
    };
    Ok(HttpResponse::Ok().json(response))
}
