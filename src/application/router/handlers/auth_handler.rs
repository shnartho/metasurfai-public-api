use crate::domain::error::AppError;
use crate::domain::model::user::User;
use crate::domain::service::user_service::UserService;
use axum::{extract::Json, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    Success(TokenResponse),
    Error(ErrorResponse),
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum SignupResponse {
    Success(SignUpSuccessResponse),
    Error(ErrorResponse),
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct SignUpRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct SignUpSuccessResponse {
    message: String,
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

pub async fn login(Json(payload): Json<LoginRequest>) -> (StatusCode, Json<LoginResponse>) {
    let user_service = UserService::new();
    let user = User::new(payload.email.clone(), payload.password.clone());
    match user_service.login_user(user).await {
        Ok(token) => (
            StatusCode::OK,
            Json(LoginResponse::Success(TokenResponse {
                message: "Login successful".to_string(),
                token,
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(LoginResponse::Error(ErrorResponse {
                error: e.to_string(),
            })),
        ),
    }
}

pub async fn signup(Json(payload): Json<SignUpRequest>) -> (StatusCode, Json<SignupResponse>) {
    let user_service = UserService::new();
    let user = User::new(payload.email.clone(), payload.password.clone());
    match user_service.signup_user(user).await {
        Ok(_) => (
            StatusCode::OK,
            Json(SignupResponse::Success(SignUpSuccessResponse {
                message: "User sign up successful".to_string(),
            })),
        ),

        Err(AppError::UserAlreadyExists) => (
            StatusCode::CONFLICT,
            Json(SignupResponse::Error(ErrorResponse {
                error: "User already exists".to_string(),
            })),
        ),

        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SignupResponse::Error(ErrorResponse {
                error: e.to_string(),
            })),
        ),
    }
}
