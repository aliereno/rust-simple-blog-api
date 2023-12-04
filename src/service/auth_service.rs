use crate::db::schema::user_login_history::dsl::*;
use crate::{db::schema::user::dsl::*, utils::hash::generate_hash};

use crate::models::user::User;
use crate::utils::error::ApiError;
use crate::utils::hash::verify_hash;
use crate::utils::token::encode_token;
use chrono::{Duration, Utc};
use diesel::prelude::*;

pub async fn user_register(
    conn: &mut SqliteConnection,
    _username: &str,
    _email: &str,
    _password: &str,
) -> Result<(), ApiError> {
    let _user: Option<User> = user
        .select(User::as_select())
        .filter(username.eq(_username))
        .first(conn)
        .optional()?;

    if _user.is_some() {
        return Err(ApiError::new(
            "User already exist.",
            actix_web::http::StatusCode::BAD_REQUEST.as_u16(),
        ));
    }

    let _password_hash = generate_hash(_password)
        .map_err(|_| {
            ApiError::new(
                "Error on generating hash!",
                actix_web::http::StatusCode::BAD_REQUEST.as_u16(),
            )
        })
        .unwrap();

    diesel::insert_into(user)
        .values((username.eq(_username), email.eq(_email), password.eq(_password_hash)))
        .execute(conn)?;

    Ok(())
}

pub async fn user_login(conn: &mut SqliteConnection, _username: &str, _password: &str) -> Result<String, ApiError> {
    let _user: User = user
        .select(User::as_select())
        .filter(username.eq(_username))
        .first(conn)?;

    if !verify_hash(_password, &_user.password).unwrap_or(false) {
        return Err(ApiError::new(
            "Password mismatch.",
            actix_web::http::StatusCode::BAD_REQUEST.as_u16(),
        ));
    }

    let _expire_at = Utc::now() + Duration::minutes(10);
    let _access_token = encode_token(_user.username, _expire_at.timestamp())
        .map_err(|_| {
            ApiError::new(
                "Error on generating token.",
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            )
        })
        .unwrap();

    diesel::insert_into(user_login_history)
        .values((
            user_id.eq(_user.id),
            token.eq(_access_token.clone()),
            expire_at.eq(_expire_at.naive_utc()),
        ))
        .execute(conn)?;

    Ok(_access_token)
}
