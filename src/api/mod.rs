use actix_web::web;
pub mod routes;
pub mod serializers;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/blog").configure(routes::blog::config_blog))
            .service(web::scope("/health").configure(routes::healtcheck::config_healthcheck)),
    );
}
