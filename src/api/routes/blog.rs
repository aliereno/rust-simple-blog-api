

use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use crate::utils::response::ApiResponse;


#[get("/blog")]  // TODO: add query filter params
pub async fn blog_list() -> Value {
    json!({ "id": 0, "title": "Hello World!", "content": "Best Blog content." })
}
#[post("/blog")]
pub async fn blog_create() -> ApiResponse<Value> {
    ApiResponse {
        json: Json(json!({ "status": format!("successfully created") })),
        status: Status::Created,
    }
}
#[get("/blog/<id>")]
pub async fn blog_detail(id: u8) -> Value {
    json!({ "blog": format!("blog #{id}") })
}
#[put("/blog/<id>")]
pub async fn blog_update(id: u8) -> Value {
    json!({ "status": format!("blog #{id}! successfully updated") })
}
#[delete("/blog/<id>")]
pub async fn blog_delete(id: u8) -> Value {
    json!({ "status": format!("blog #{id}! successfully deleted") })
}