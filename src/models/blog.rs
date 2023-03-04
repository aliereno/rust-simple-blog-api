use crate::db::schema::blogs;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = blogs)]
pub struct Blog {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
