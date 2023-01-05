use actix_web::web;
pub mod routes;
pub mod serializers;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    // domain includes: /products/{product_id}/parts/{part_id}
    cfg
    .service(routes::blog::blog_detail)
    .service(routes::blog::blog_update)
    .service(routes::blog::blog_delete)
    .service(routes::blog::blog_create)
    .service(routes::blog::blog_list);
}