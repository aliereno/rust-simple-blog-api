use actix_web::{get, web, HttpResponse, Result};

use crate::api::serializers::HealthCheckOut;
use crate::utils::error::ApiError;

pub fn config_healthcheck(cfg: &mut web::ServiceConfig) {
    cfg.service(healthcheck);
}

#[get("/")]
pub async fn healthcheck() -> Result<HttpResponse, ApiError> {
    let response = HealthCheckOut {
        status: "available".to_string(),
    };
    Ok(HttpResponse::Ok().json(response))
}
