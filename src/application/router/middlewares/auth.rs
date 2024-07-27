use axum::{
    async_trait,
    extract::FromRequest,
    http::{StatusCode, Request},
};
use crate::application::router::middlewares::jwt::validate_token;

#[derive(Clone, Debug)]
pub struct Auth;

#[async_trait]
impl<S, B> FromRequest<S, B> for Auth
where
    S: Send + Sync,
    B: Send + 'static,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = req.headers();
        if let Some(auth_header) = headers.get("Authorization") {
            if let Ok(auth_header_str) = auth_header.to_str() {
                if let Some(token) = auth_header_str.strip_prefix("Bearer ") {
                    match validate_token(token) {
                        Ok(_) => return Ok(Auth),
                        Err(_) => return Err((StatusCode::UNAUTHORIZED, "Invalid token".into())),
                    }
                }
            }
        }
        Err((StatusCode::UNAUTHORIZED, "Missing or invalid authorization header".into()))
    }
}
