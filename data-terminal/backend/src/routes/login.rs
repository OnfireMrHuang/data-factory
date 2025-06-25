use axum::{
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    routing::post,
    extract::{Json, Extension},
    Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use bcrypt::{verify};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use crate::utils::config::Setting;

static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn routes() -> Router {
    Router::new()
        .route("/", post(login))
}


async fn login(
    Json(payload): Json<web::LoginRequest>,
) -> (StatusCode, Json<web::Response<String>>) {

    let setting = Setting::get();
    let admin = &setting.admin;

    // 校验用户名
    if payload.username != admin.username {
        return (StatusCode::UNAUTHORIZED, "用户名或密码错误").into_response();
    }

    // 校验密码（bcrypt）
    if !verify(&payload.password, &admin.password).unwrap_or(false) {
        return (StatusCode::UNAUTHORIZED, "用户名或密码错误").into_response();
    }

    // 生成 JWT
    let exp = (Utc::now() + Duration::hours(24)).timestamp() as usize;
    let claims = Claims {
        sub: admin.username.clone(),
        exp,
    };
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error")).unwrap();

    // 设置 Cookie
    let cookie = Cookie::build("token", token.clone())
        .http_only(true)
        .path("/")
        .max_age(time::Duration::days(1))
        .finish();

    let jar = jar.add(cookie);

    // 返回响应
    (
        jar,
        Json(web::Response {
            result: true,
            msg: "Login successful".to_string(),
            data: token,
        })
    ).into_response()
}