use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{post},
    Json, Router,
};
use axum_extra::{
    extract::cookie::{Cookie, CookieJar},
};
use cookie::time::Duration;
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration as ChronoDuration};
use crate::utils::config::Setting;
use crate::models::web;
use super::jwt::Claims;


pub fn routes() -> Router {
    Router::new()
        .route("/", post(login))
}


async fn login(
    jar: CookieJar,
    Json(payload): Json<web::LoginRequest>,
) -> impl IntoResponse {
    let setting = Setting::get();
    let admin = &setting.admin;

    // 校验用户名
    if payload.username != admin.username {
        return (
            StatusCode::UNAUTHORIZED,
            jar,
            Json(web::Response::<String> {
                result: false,
                msg: "username is incorrect".to_string(),
                data: "".to_string(),
            }),
        );
    }

    if admin.password != payload.password {
        return (
            StatusCode::UNAUTHORIZED,
            jar,
            Json(web::Response::<String> {
                result: false,
                msg: "password is incorrect".to_string(),
                data: "".to_string(),
            }),
        );
    }

    // 生成 JWT
    let exp = (Utc::now() + ChronoDuration::hours(24)).timestamp() as usize;
    let claims = Claims {
        sub: admin.username.clone(),
        company: "".to_string(),
        project: "".to_string(),
        exp,
    };
    let secret = Setting::get().jwt.secret.clone();
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                jar.clone(),
                Json(web::Response::<String> {
                    result: false,
                    msg: "creating token occur error".to_string(),
                    data: "".to_string(),
                }),
            )
        })
        .unwrap();

    // 设置 Cookie
    let cookie = Cookie::build(("token", token.clone()))
        .http_only(true)
        .path("/")
        .max_age(Duration::days(1));
    let jar = jar.add(cookie);

    (
        StatusCode::OK,
        jar,
        Json(web::Response::<String> {
            result: true,
            msg: "login success".to_string(),
            data: "".to_string(),
        }),
    )
}

