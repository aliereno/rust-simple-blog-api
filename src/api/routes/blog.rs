
use actix_web::{get, post, put, delete, web, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct MessageOut {
    message: String,
}
#[derive(Serialize)]
struct BlogOut {
    id: u32,
    title: String,
}


#[get("/blog")]  // TODO: add query filter params
pub async fn blog_list() -> Result<impl Responder> {
    let response = [BlogOut {
        id: 1,
        title: format!("title for blog 1"),
    }, BlogOut {
        id: 2,
        title: format!("title for blog 2"),
    }];
    Ok(web::Json(response))
}
#[post("/blog")]
pub async fn blog_create() -> Result<impl Responder> {
    let response = MessageOut {
        message: "successfully created".to_string(),
    };
    Ok(web::Json(response))
}
#[get("/blog/{id}")]
pub async fn blog_detail(_id: web::Path<u32>) -> Result<impl Responder> {
    let response = BlogOut {
        id: *_id,
        title: format!("title for blog #{_id}"),
    };
    Ok(web::Json(response))
}
#[put("/blog/{id}")]
pub async fn blog_update(_id: web::Path<u32>) -> Result<impl Responder> {
    let response = MessageOut {
        message: format!("blog #{_id} successfully updated"),
    };
    Ok(web::Json(response))
}
#[delete("/blog/{id}")]
pub async fn blog_delete(_id: web::Path<u32>) -> Result<impl Responder> {
    let response = MessageOut {
        message: format!("blog #{_id} successfully deleted"),
    };
    Ok(web::Json(response))
}