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

#[derive(Serialize, Deserialize)]
pub struct RegisterIn {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginIn {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginOut {
    pub access_token: String,
}
