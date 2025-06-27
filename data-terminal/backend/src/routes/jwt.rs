use axum::{
    extract::{FromRequestParts, FromRequest},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::{
    extract::cookie::CookieJar,
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey,  Validation};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use std::fmt::Display;
use crate::utils::config::Setting;
use crate::models::web;



static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    let secret = Setting::get().jwt.secret.clone();
    Keys::new(secret.as_bytes())
});



#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(web::Response::<()> {
            result: false,
            msg: error_message.to_string(),
            data: (),
        });
        (status, body).into_response()
    }
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub project: String,
    pub exp: usize,
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email: {}\nCompany: {}\nProject: {}", self.sub, self.company, self.project)
    }
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {

        let jar = CookieJar::from_headers(&parts.headers);
        let token = jar
            .get("token")
            .map(|cookie| cookie.value().to_string())
            .ok_or(AuthError::MissingCredentials)?;

        // 校验 token
        let token_data = decode::<Claims>(&token, &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}