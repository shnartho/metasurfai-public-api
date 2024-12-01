use crate::application::router::middlewares::auth::Auth;
use crate::domain::service::profile_service::ProfileService;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use crate::application::AppState;

pub async fn profile_handler(Extension(app_state): Extension<AppState>, auth: Auth) -> impl IntoResponse {
    let profile_service = app_state.profile_service.clone();
    
    match profile_service
        .get_profile_from_db(auth.user_email.clone())
        .await
    {
        Ok(Some(profile)) => {
            let profile_json = serde_json::to_string(&profile).unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(axum::body::Body::from(profile_json))
                .unwrap()
        }
        Ok(None) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(axum::body::Body::from("Profile not found"))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::from("Failed to get profile"))
            .unwrap(),
    }
}
