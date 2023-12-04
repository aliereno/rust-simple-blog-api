use crate::db::schema::blog::dsl::*;
use crate::models::blog::Blog;
use crate::utils::error::ApiError;
use diesel::prelude::*;

pub async fn get_all_blogs(conn: &mut SqliteConnection) -> Result<Vec<Blog>, ApiError> {
    let items = blog
        .select(Blog::as_select())
        .filter(published.eq(true))
        .load::<Blog>(conn)?;
    Ok(items)
}

pub async fn get_blog_by_id(conn: &mut SqliteConnection, _id: i32) -> Result<Blog, ApiError> {
    let item = blog
        .select(Blog::as_select())
        .filter(published.eq(true))
        .find(_id)
        .first(conn)?;

    Ok(item)
}

pub async fn create_blog(
    conn: &mut SqliteConnection,
    _title: &str,
    _body: &str,
    _published: &bool,
) -> Result<(), ApiError> {
    diesel::insert_into(blog)
        .values((title.eq(_title), body.eq(_body), published.eq(_published)))
        .execute(conn)?;

    Ok(())
}

pub async fn update_blog(
    conn: &mut SqliteConnection,
    _id: i32,
    _title: &str,
    _body: &str,
    _published: &bool,
) -> Result<Option<Blog>, ApiError> {
    let _blog = diesel::update(blog.find(_id))
        .set((title.eq(_title), body.eq(_body), published.eq(_published)))
        .returning(Blog::as_returning())
        .get_result::<Blog>(conn)?;

    Ok(Some(_blog))
}

pub async fn delete_blog(conn: &mut SqliteConnection, _id: i32) -> Result<(), ApiError> {
    diesel::delete(blog.filter(id.eq(_id))).execute(conn)?;

    Ok(())
}
