
use diesel::prelude::*;
use crate::{models::blog::Blog};

pub type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_all_blogs(conn: &mut PgConnection)-> Result<Vec<Blog>, DbError> {
    use crate::db::schema::blogs::dsl::*;

    let items = blogs
        // .filter(published.eq(true))
        .load::<Blog>(conn)?;

    Ok(items)
}

pub fn get_blog_by_id(conn: &mut PgConnection, _id: i32)-> Result<Option<Blog>, DbError> {
    use crate::db::schema::blogs::dsl::*;

    let item = blogs
        .find(_id)
        .first(conn)
        .optional()?;

    Ok(item)
}

pub fn create_blog(conn: &mut PgConnection, _title: &str, _body: &str, _published: &bool) -> Result<(), DbError> {
    use crate::db::schema::blogs::dsl::*;

    diesel::insert_into(blogs).values((title.eq(_title), body.eq(_body), published.eq(_published))).execute(conn)?;

    Ok(())
}

pub fn update_blog(conn: &mut PgConnection, _id: i32, _title: &str, _body: &str, _published: &bool) -> Result<Option<Blog>, DbError> {
    use crate::db::schema::blogs::dsl::*;

    let blog = diesel::update(blogs.find(_id))
        .set((title.eq(_title), body.eq(_body), published.eq(_published)))
        .get_result::<Blog>(conn)?;
    
    Ok(Some(blog))
}

pub fn delete_blog(conn: &mut PgConnection, _id: i32)-> Result<(), DbError> {
    use crate::db::schema::blogs::dsl::*;

    diesel::delete(blogs.filter(id.eq(_id)))
        .execute(conn)
        .expect("Error deleting posts");

    Ok(())
}