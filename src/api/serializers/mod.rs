use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct MessageOut {
    pub message: String,
}

#[derive(Serialize)]
pub struct HealthCheckOut {
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct BlogIn {
    pub title: String,
    pub body: String,
    pub published: bool,
}
