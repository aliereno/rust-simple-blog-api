use actix_service::ServiceFactory;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use actix_web::{middleware, web, App, Error};

use crate::api;
use crate::db::DBPool;
use crate::utils::error::ApiError;

pub fn create_app(
    db_pool: &DBPool,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<impl MessageBody>,
        Error = Error,
        InitError = (),
    >,
> {
    App::new()
        .wrap(middleware::DefaultHeaders::new().add(("Content-Type", "application/json")))
        .app_data(web::Data::new(db_pool.clone()))
        .app_data(
            web::JsonConfig::default()
                .error_handler(|f: actix_web::error::JsonPayloadError, _| ApiError::new(&f.to_string(), 422).into()),
        )
        .configure(api::config_app)
        .wrap(Logger::default())
}
