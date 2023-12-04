
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};

static SECRET_KEY: &str = "MY_SECRET_KEY"; // TODO: get from config

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthToken {
    exp: i64,
    username: String,
}

pub fn encode_token(username: String, exp: i64) -> jsonwebtoken::errors::Result<String> {
    let _now = Utc::now().timestamp();

    jsonwebtoken::encode::<AuthToken>(
        &Header::default(),
        &AuthToken { exp, username }, // TODO: get expire from config
        &EncodingKey::from_secret(SECRET_KEY.as_ref()),
    )
}
