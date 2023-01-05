
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct MessageOut {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct BlogIn {
    pub title: String,
    pub body: String,
    pub published: bool,
}