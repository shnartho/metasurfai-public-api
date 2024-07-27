use crate::domain::service::auth_service::authenticate;
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    Success(TokenResponse),
    Error(ErrorResponse),
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    message: String,
    token: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

pub async fn login(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    match authenticate(&payload.username[..], &payload.password[..]) {
        Ok(token) => (
            StatusCode::OK,
            Json(LoginResponse::Success(TokenResponse {
                message: "success".to_string(),
                token,
            })),
        ),
        Err(e) => (
            StatusCode::UNAUTHORIZED,
            Json(LoginResponse::Error(ErrorResponse {
                error: e.to_string(),
            })),
        ),
    }
}