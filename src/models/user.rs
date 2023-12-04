use crate::db::schema::{user, user_login_history};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Selectable, Queryable, Insertable)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Selectable, Queryable, Insertable)]
#[diesel(table_name = user_login_history)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserLoginHistory {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub expire_at: NaiveDateTime,
}
