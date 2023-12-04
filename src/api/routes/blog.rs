use actix_web::{delete, get, post, put, web, HttpResponse, Result};

use crate::api::serializers::{BlogIn, MessageOut};
use crate::db::DBPool;
use crate::service::blog_service;
use crate::utils::connection::get_conn;
use crate::utils::error::ApiError;

pub fn config_blog(cfg: &mut web::ServiceConfig) {
    cfg.service(blog_detail)
        .service(blog_update)
        .service(blog_delete)
        .service(blog_create)
        .service(blog_list);
}

#[get("/")] // TODO: add query filter params
pub async fn blog_list(db_pool: web::Data<DBPool>) -> Result<HttpResponse, ApiError> {
    let mut conn = get_conn(db_pool).await?;
    let blogs = blog_service::get_all_blogs(&mut conn).await?;
    Ok(HttpResponse::Ok().json(blogs))
}

#[post("/")]
pub async fn blog_create(new_blog: web::Json<BlogIn>, db_pool: web::Data<DBPool>) -> Result<HttpResponse, ApiError> {
    let mut conn = get_conn(db_pool).await?;
    blog_service::create_blog(&mut conn, &new_blog.title, &new_blog.body, &new_blog.published).await?;
    let response = MessageOut {
        message: "successfully created".to_string(),
    };
    Ok(HttpResponse::Ok().json(response))
}

#[get("/{id}")]
pub async fn blog_detail(_id: web::Path<i32>, db_pool: web::Data<DBPool>) -> Result<HttpResponse, ApiError> {
    let mut conn = get_conn(db_pool).await?;
    let blog = blog_service::get_blog_by_id(&mut conn, *_id).await?;
    Ok(HttpResponse::Ok().json(blog))
}

#[put("/{id}")]
pub async fn blog_update(
    _id: web::Path<i32>,
    update_blog: web::Json<BlogIn>,
    db_pool: web::Data<DBPool>,
) -> Result<HttpResponse, ApiError> {
    let mut conn = get_conn(db_pool).await?;
    blog_service::get_blog_by_id(&mut conn, *_id).await?;
    blog_service::update_blog(
        &mut conn,
        *_id,
        &update_blog.title,
        &update_blog.body,
        &update_blog.published,
    )
    .await?;

    let response = MessageOut {
        message: "successfully updated".to_string(),
    };
    Ok(HttpResponse::Ok().json(response))
}

#[delete("/{id}")]
pub async fn blog_delete(_id: web::Path<i32>, db_pool: web::Data<DBPool>) -> Result<HttpResponse, ApiError> {
    let mut conn = get_conn(db_pool).await?;
    blog_service::get_blog_by_id(&mut conn, *_id).await?;
    blog_service::delete_blog(&mut conn, *_id).await?;
    let response = MessageOut {
        message: "successfully deleted".to_string(),
    };
    Ok(HttpResponse::Ok().json(response))
}
