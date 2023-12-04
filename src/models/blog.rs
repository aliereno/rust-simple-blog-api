use crate::db::schema::blog;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Selectable, Queryable, Insertable)]
#[diesel(table_name = blog)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Blog {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
