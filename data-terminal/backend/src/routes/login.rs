use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    routing::{post},
    Json, Router,
};
use axum_extra::{
    extract::cookie::{Cookie, CookieJar},
};
use cookie::time::Duration;
use bcrypt::{verify};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration as ChronoDuration};
use std::sync::LazyLock;
use std::fmt::Display;
use crate::utils::config::Setting;
use crate::models::web;

static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    let secret = Setting::get().jwt.secret.clone();
    Keys::new(secret.as_bytes())
});


pub fn routes() -> Router {
    Router::new()
        .route("/", post(login))
}


async fn login(
    jar: CookieJar,
    Json(payload): Json<web::LoginRequest>,
) -> (StatusCode, Json<web::Response<String>>) {

    let setting = Setting::get();
    let admin = &setting.admin;

    // 校验用户名
    if payload.username != admin.username {
        return (StatusCode::UNAUTHORIZED, Json(web::Response::<String> {
            result: false,
            msg: "username is incorrect".to_string(),
            data: "".to_string(),
        }));
    }

    // 校验密码（bcrypt）
    // 将配置中的明文密码加密后，与前端传过来的加密密码进行校验
    let admin_password_hash = bcrypt::hash(&admin.password, bcrypt::DEFAULT_COST).unwrap();
    if !verify(&admin_password_hash, &payload.password).unwrap_or(false) {
        return (StatusCode::UNAUTHORIZED, Json(web::Response::<String> {
            result: false,
            msg: "password is incorrect".to_string(),
            data: "".to_string(),
        }));
    }

    // 生成 JWT
    let exp = (Utc::now() + ChronoDuration::hours(24)).timestamp() as usize;
    let claims = Claims {
        sub: admin.username.clone(),
        company: "".to_string(),
        exp,
    };
    let secret = Setting::get().jwt.secret.clone();
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(web::Response::<String>{
            result: false,
            msg: "creating token occur error".to_string(),
            data: "".to_string(),
        }))).unwrap();

    // 设置 Cookie
    let cookie = Cookie::build(("token", token.clone()))
        .http_only(true)
        .path("/")
        .max_age(Duration::days(1));

    jar.add(cookie);

    (StatusCode::OK, Json(web::Response::<String>{
        result: true,
        msg: "login success".to_string(),
        data: "".to_string(),  
    }))
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

#[derive(Debug)]
enum AuthError {
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


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email: {}\nCompany: {}", self.sub, self.company)
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