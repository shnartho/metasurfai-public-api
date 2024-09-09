use crate::domain::service::profile_service::get_profile;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProfileQuery {
    pub username: String,
}

pub async fn profile_handler(Query(params): Query<ProfileQuery>) -> impl IntoResponse {
    match get_profile(&params.username).await {
        Some(profile) => {
            let profile_json = serde_json::to_string(&profile).unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(axum::body::Body::from(profile_json))
                .unwrap()
        }
        None => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::from("Failed to get profile"))
            .unwrap(),
    }
}
