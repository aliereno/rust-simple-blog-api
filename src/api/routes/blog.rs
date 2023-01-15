
use actix_web::{get, post, put, delete, web, Error, HttpResponse, Responder, Result};

use crate::db::DBPool;
use crate::api::serializers::{MessageOut, BlogIn};
use crate::service::blog_service;


pub fn config_blog(cfg: &mut web::ServiceConfig) {
    cfg
    .service(blog_detail)
    .service(blog_update)
    .service(blog_delete)
    .service(blog_create)
    .service(blog_list);
}

#[get("/")]  // TODO: add query filter params
pub async fn blog_list(db_pool: web::Data<DBPool>) -> Result<HttpResponse, Error> {

    let response = web::block(move || {
        let mut conn = db_pool.get()?;

        blog_service::get_all_blogs(&mut conn)
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;
    
    Ok(HttpResponse::Ok().json(response))
}

#[post("/")]
pub async fn blog_create(new_blog: web::Json<BlogIn>, db_pool: web::Data<DBPool>) -> Result<impl Responder> {

    web::block(move || {
        let mut conn = db_pool.get()?;

        blog_service::create_blog(&mut conn, &new_blog.title, &new_blog.body, &new_blog.published)
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;

    let response = MessageOut {
        message: "successfully created".to_string(),
    };
    Ok(web::Json(response))
}

#[get("/{id}")]
pub async fn blog_detail(_id: web::Path<i32>, db_pool: web::Data<DBPool>) -> Result<HttpResponse, Error> {

    let response = web::block(move || {
        let mut conn = db_pool.get()?;

        blog_service::get_blog_by_id(&mut conn, _id.into_inner())
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;
    
    if response.is_none() {
        let response = MessageOut {
            message: format!("blog not found"),
        };
        return Ok(HttpResponse::NotFound().json(response))
    }
    Ok(HttpResponse::Ok().json(response))
}

#[put("/{id}")]
pub async fn blog_update(_id: web::Path<i32>, update_blog: web::Json<BlogIn>, db_pool: web::Data<DBPool>) -> Result<HttpResponse, Error> {

    web::block(move || {
        let mut conn = db_pool.get()?;

        blog_service::update_blog(&mut conn, _id.into_inner(), &update_blog.title, &update_blog.body, &update_blog.published)
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;
    

    let response = MessageOut {
        message: format!("successfully updated"),
    };
    Ok(HttpResponse::Ok().json(response))
}

#[delete("/{id}")]
pub async fn blog_delete(_id: web::Path<i32>, db_pool: web::Data<DBPool>) -> Result<HttpResponse, Error> {
    web::block(move || {
        let mut conn = db_pool.get()?;

        blog_service::delete_blog(&mut conn, _id.into_inner())
      })
      .await?
      .map_err(actix_web::error::ErrorInternalServerError)?;
    
    let response = MessageOut {
        message: format!("successfully deleted"),
    };
    Ok(HttpResponse::Ok().json(response))
}
