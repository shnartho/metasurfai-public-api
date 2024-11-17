use crate::application::router::middlewares::jwt::validate_token;
use axum::async_trait;
use axum::extract::FromRequest;
use axum::http::{Request, StatusCode}; // Make sure validate_token is in scope

#[derive(Clone, Debug)]
pub struct Auth {
    pub user_email: String, // Capture the user's email from the token
}

#[async_trait]
impl<S, B> FromRequest<S, B> for Auth
where
    S: Send + Sync,
    B: Send + 'static,
{
    type Rejection = (StatusCode, String); // Reject with status and message if something goes wrong

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = req.headers();

        if let Some(auth_header) = headers.get("Authorization") {
            if let Ok(auth_header_str) = auth_header.to_str() {
                if let Some(token) = auth_header_str.strip_prefix("Bearer ") {
                    // Validate the token and check expiration
                    match validate_token(token) {
                        Ok(claims) => {
                            // If valid, return Auth with extracted user email
                            return Ok(Auth {
                                user_email: claims.sub,
                            });
                        }
                        Err(err) => {
                            // Token is invalid or expired
                            return Err((
                                StatusCode::UNAUTHORIZED,
                                format!("Invalid token: {}", err),
                            ));
                        }
                    }
                }
            }
        }
        // Missing or malformed authorization header
        Err((
            StatusCode::UNAUTHORIZED,
            "Missing or invalid authorization header".into(),
        ))
    }
}
