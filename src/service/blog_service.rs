use crate::db::schema::blogs::dsl::*;
use crate::models::blog::Blog;
use crate::utils::error::ApiError;
use diesel::prelude::*;

pub struct BlogService {}

impl BlogService {
    pub async fn get_all_blogs(conn: &mut PgConnection) -> Result<Vec<Blog>, ApiError> {
        let items = blogs.filter(published.eq(true)).load::<Blog>(conn)?;

        Ok(items)
    }

    pub async fn get_blog_by_id(conn: &mut PgConnection, _id: i32) -> Result<Blog, ApiError> {
        let item = blogs.filter(published.eq(true)).find(_id).first(conn)?;

        Ok(item)
    }

    pub async fn create_blog(
        conn: &mut PgConnection,
        _title: &str,
        _body: &str,
        _published: &bool,
    ) -> Result<(), ApiError> {
        diesel::insert_into(blogs)
            .values((title.eq(_title), body.eq(_body), published.eq(_published)))
            .execute(conn)?;

        Ok(())
    }

    pub async fn update_blog(
        conn: &mut PgConnection,
        _id: i32,
        _title: &str,
        _body: &str,
        _published: &bool,
    ) -> Result<Option<Blog>, ApiError> {
        let blog = diesel::update(blogs.find(_id))
            .set((title.eq(_title), body.eq(_body), published.eq(_published)))
            .get_result::<Blog>(conn)?;

        Ok(Some(blog))
    }

    pub async fn delete_blog(conn: &mut PgConnection, _id: i32) -> Result<(), ApiError> {
        diesel::delete(blogs.filter(id.eq(_id))).execute(conn)?;

        Ok(())
    }
}
