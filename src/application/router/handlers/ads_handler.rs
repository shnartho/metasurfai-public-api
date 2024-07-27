use crate::domain::service::ads_service::get_ads_from_db;
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

pub async fn ads_handler() -> impl IntoResponse {
    match get_ads_from_db().await {
        Ok(ads) => {
            let ads_json = serde_json::to_string(&ads).unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(axum::body::Body::from(ads_json))
                .unwrap()
        }
        Err(_) => {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(axum::body::Body::from("Failed to get ads"))
                .unwrap()
        }
    }
}
