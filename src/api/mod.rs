
use rocket::serde::json::{json, Value};

mod routes;

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount(
            "/api",
            routes![
                routes::blog::blog_list,
                routes::blog::blog_detail,
                routes::blog::blog_create,
                routes::blog::blog_update,
                routes::blog::blog_delete,
            ],
        )
        .register("/", catchers![not_found])
}